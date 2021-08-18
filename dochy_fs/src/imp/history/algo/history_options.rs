use crate::error::FsResult;
use once_cell::sync::Lazy;

/// Customize how we build history files.
///
/// If you need the default value, you can get it with new()
/// ```
/// use dochy_fs::history::HistoryOptions;
/// let op = HistoryOptions::new();
/// ```
/// If you want to customize, you can use builders.
///
/// Appending "..Default::default()" could make your source code
/// compatible with future versions of this library.
/// ```
/// use dochy_fs::error::FsResult;
/// use dochy_fs::history::{HistoryOptions, HistoryOptionsBuilder, CumulativeOptionsBuilder};
///
/// fn main() -> FsResult<()>{
/// let op = HistoryOptions::from(
///         HistoryOptionsBuilder {
///            max_phase: 5,
///            cumulative: Some(CumulativeOptionsBuilder {
///                limit_nth: Some(2),
///                limit_count: Some(100),
///                 ..Default::default()
///            }),
///            ..Default::default()
///        })?;
///     Ok(())
/// }
/// ```
/// You need to use the same options for a history_hash_dir.
/// Changing the value causes an undefined behavior.
///
/// When you modify the source Dochy file, a new history_hash_dir will be created,
/// so you can change the options at the very time.
///
/// You can use () for AsRef<HistoryOptions> to assign the default value.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryOptions {
    max_phase : usize,
    update_phase_0: bool,
    cumulative : Option<CumulativeOptions>,
    mt_save : Option<Option<usize>>,
    mt_load : Option<Option<usize>>,
}

impl HistoryOptions {

    /// if max_phase == 1, there will be no Phase-2.
    pub fn max_phase(&self) -> usize{ self.max_phase }

    ///if false, Phase-0 isn't made twice
    pub fn update_phase_0(&self) -> bool{ self.update_phase_0 }

    /// If this is Some, the max phase will be cumulative
    pub fn cumulative(&self) -> Option<&CumulativeOptions>{
        self.cumulative.as_ref()
    }

    /// self.cumulative().is_some()
    pub fn is_cumulative(&self) -> bool{ self.cumulative.is_some() }

    /// enables multi-thread save
    pub fn mt_save(&self) -> bool{ self.mt_save.is_some() }

    /// If None, default or not multi-threaded
    pub fn num_save_threads(&self) -> Option<usize>{ self.mt_save.flatten() }

    /// enables multi-thread load
    pub fn mt_load(&self) -> bool{ self.mt_load.is_some() }

    /// If None, default or not multi-threaded
    pub fn num_load_threads(&self) -> Option<usize>{ self.mt_load.flatten() }

    /// Construct HistoryOption with default values
    pub fn new() -> HistoryOptions {
        Self::from(Default::default()).unwrap()
    }

    /// Construct HistoryOption from builders
    pub fn from(builder : HistoryOptionsBuilder) -> FsResult<HistoryOptions>{
        if builder.max_phase == 0{
            if builder.update_phase_0 == false {
                return Err(format!("max_phase == 0 && update_phase_a == false is inconsistent."))?;
            }
            if let Some(cumu) = &builder.cumulative{
                if let Some(_) = cumu.limit_nth{
                    return Err(format!("max_phase == 0 && limit_nth is inconsistent."))?;
                }
                if let Some(_) = cumu.limit_count{
                    return Err(format!("max_phase == 0 && limit_count is inconsistent."))?;
                }
            }
        }
        Ok(HistoryOptions {
            max_phase : builder.max_phase,
            update_phase_0: builder.update_phase_0,
            mt_save: builder.mt_save,
            mt_load: builder.mt_load,
            cumulative : builder.cumulative
                .map(|c| CumulativeOptions::from(c)).transpose()?
        })
    }
}

/// Customize how "cumulative diff files" are built
#[derive(Debug, Clone, PartialEq)]
pub struct CumulativeOptions {
    limit_nth : Option<usize>,
    limit_count : Option<usize>,
}

impl CumulativeOptions {
    /// The total size of a Cumulative-Phase must be less than nth largest diff file in its ancestors
    ///
    /// Basically the most largest file should be phase-A. the 2nd largest should be Phase-B...
    pub fn limit_nth(&self) -> Option<usize>{ self.limit_nth }

    /// The total number of diff files in a Cumulative-Phase must be fewer than this
    pub fn limit_count(&self) -> Option<usize>{ self.limit_count }

    fn from(builder : CumulativeOptionsBuilder) -> FsResult<CumulativeOptions>{
        if Some(0) == builder.limit_nth{
            Err("invalid argument: limit_nth=0")?
        }
        if Some(0) == builder.limit_count{
            Err("invalid argument: limit_count=0")?
        }
        Ok(CumulativeOptions { limit_nth : builder.limit_nth, limit_count : builder.limit_count })
    }
}

///Construct HistoryOption with error check
#[derive(Debug, Clone)]
pub struct HistoryOptionsBuilder {
    /// Phase-A=0, Phase-B=1...
    /// if max_phase == 1, there will be no Phase-C.
    pub max_phase : usize,

    ///if false, Phase-0 isn't made twice
    pub update_phase_0 : bool,

    /// If this is Some, the max phase will be cumulative
    pub cumulative : Option<CumulativeOptionsBuilder>,

    /// If None, not multi-threaded,
    /// If Some(None), default multi-threaded,
    /// If Some(Some(num)) multi-threaded with num threads
    pub mt_save : Option<Option<usize>>,

    /// If None, not multi-threaded,
    /// If Some(None), default multi-threaded,
    /// If Some(Some(num)) multi-threaded with num threads
    pub mt_load : Option<Option<usize>>,
}

impl Default for HistoryOptionsBuilder {
    fn default() -> Self {
        Self{
            max_phase : 3,
            update_phase_0 : true,
            mt_save : None,
            mt_load : Some(None),
            cumulative : Some(CumulativeOptionsBuilder::default()),
        }
    }
}

///Construct CumulativeOption with error check
#[derive(Debug, Clone)]
pub struct CumulativeOptionsBuilder {
    ///The total size of a Cumulative-Phase must be less than nth largest diff file in its ancestors
    ///
    /// Basically the most largest file should be Phase-A. the 2nd largest should be Phase-B...
    pub limit_nth : Option<usize>,

    /// The total number of diff files in a Cumulative-phase must be fewer than this
    pub limit_count : Option<usize>,
}

impl Default for CumulativeOptionsBuilder {
    fn default() -> Self {
        CumulativeOptionsBuilder {
            limit_nth : Some(1),
            limit_count : Some(10),
        }
    }
}

impl AsRef<HistoryOptions> for HistoryOptions{
    fn as_ref(&self) -> &HistoryOptions {
        &self
    }
}

impl AsRef<HistoryOptions> for (){
    fn as_ref(&self) -> &HistoryOptions {
        static DEF : Lazy<HistoryOptions> = Lazy::new(|| HistoryOptions::new());
        DEF.as_ref()
    }
}
