use crate::imp::structs::linked_m::{LinkedMap};
use crate::imp::structs::rust_list::MutItem;
use std::marker::PhantomData;
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::intf::mitem_ptr::MItemPtr;
use crate::imp::structs::root_def_obj::RootDefObj;
use crate::imp::intf::mitem_cptr::MItemCPtr;
use crate::imp::structs::linked_map_unsafe_citer::LinkedMapUnsafeCIter;
use crate::imp::intf::mlist_cptr_iter::MListCPtrIter;


/// &参照から得られたポインタを*constを経て&mut参照にする行為の安全性に確信が持てなかった。
/// 最初に&mutから*mutを取り、*mutから&参照を取得、*constから*mutにキャストして&mutを取り出す行為は安全だろうか。
/// たとえmutationを伴わなくても、&mut参照に対して aliasing を行えばUBになる。https://stackoverflow.com/questions/54237610/is-there-a-way-to-make-an-immutable-reference-mutable
///
/// このシステムでは参照を露出しないので、シングルスレッドアクセスでは1つのメモリアドレスへの2つの参照が同時に存在することはないようになっている（はず
/// しかし、パフォーマンス上の利益が非常に大きいため、Arcのmake_mutを介して、マルチスレッドアクセスを解禁することになった。
/// Arcを介しているので、mut参照と&参照が同時に存在することはできないはず
///
/// MListPtrは&mutから*mutを取り出すことで作成されるが、Arc::make_mutを起動してしまう。
/// これではパフォーマンス上のメリットがほとんど受けられない
/// &から得た*constポインタを*mutポインタにして無理やりMListPtrにしても別に問題はない気はする。
/// ただArcの制限をかいくぐってしまう。そして、たとえmutateしないとしても、
/// &mut self呼び出しのときなんかに一時的に&mutが作られてしまう可能性がある（あるか？
/// マルチスレッドアクセスにより、&mutと&参照が同時に存在することがありうるように思う
///
/// 一つの関数が２つの引数をとり、その２つが&mut T と &Tだったとき、2つは同一のアドレスを指していないと仮定されるはず
/// mutateしないという条件でもそれが問題になる可能性もないわけではないように思う（わからん・・・
/// しかし一つの関数内ではなくマルチスレッドの話なので、それが問題になることはないはずだ。
/// &参照が生きてる間にmutationが行われればもちろんUBだし、Arcの中身をmake_mutを介さずmutateすればすべてが崩壊するのは確かだ
/// そしてMListPtrを&から得ればそれらへの扉がガバっと開いてしまうので、冷静に考えて許容できない
/// しかし、mutateなしでのMListPtrの作成がUBなのかはいまだ判然としない。
/// とにかく、UBかどうかは知らないが、mutateがなしえないバージョンのMListPtrを作ることになった。それがMListCPtr
///
/// 考えなければいけないのは、MListCPtrが生きている間にArc::make_mutが起きて、クローンされてないと判定されること。
/// 通常&参照が生きている間はmake_mutは呼び出せないが、ポインタしか取らないMListCPtrではその制限は発生しない。
/// マルチスレッドアクセスにより書き換え途中の不正な値がgetされてしまう可能性を考慮する必要があるように見える
///
/// また、MlistCPtrが存在している間にArc::make_mutが発生し、クローンされたと判定される場合、make_mutで新しく作られた方ではなく、
/// 古い方のデータを参照してしまうことになる。それはMListPtrの場合は、MListPtrを作ろうとした瞬間
/// Arc::make_mutが起きて新しい方になるので、その問題は起こらないだろう
///
/// Safe Wrapperを介していればこれらの問題は起こらないはずだが、それ以外の場合はMListCPtrの参照先がmutateされないようにしなければならない
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MListCPtr<V : From<MItemCPtr>>{
    map : *const LinkedMap<MutItem>,
    list_def : *const ListDefObj,
    root_def : *const RootDefObj,
    phantom : PhantomData<*const V>,
}

impl<V : From<MItemCPtr>> MListCPtr<V>{
    pub fn new(map : *const LinkedMap<MutItem>, list_def : *const ListDefObj, root_def : *const RootDefObj)
        -> MListCPtr<V>{ MListCPtr { map, list_def, root_def, phantom : PhantomData } }

    fn from(&self, item : *const MutItem) -> V{
        V::from(MItemCPtr::new(item, self.list_def, self.root_def))
    }

    pub fn first(&self) -> Option<V> {
        let map = unsafe{ &*self.map };
        map.first().map(|r| self.from(r))
    }
    pub fn first_id(&self) -> Option<u64> {
        let map = unsafe{ &*self.map };
        map.first_id()
    }
    pub fn last(&self) -> Option<V> {
        let map = unsafe{ &*self.map };
        map.last().map(|r| self.from(r))
    }


    pub fn last_id(&self) -> Option<u64> {
        let map = unsafe{ &*self.map };
        map.last_id()
    }
    pub fn get_item(&self, id : u64) -> Option<V>{
        let map = unsafe{ &*self.map };
        map.get_item(id).map(|b| self.from(b))
    }

    pub fn next_id(&self) -> u64{
        let map = unsafe{ &*self.map };
        map.next_id()
    }

    pub fn contains_key(&self, key : u64) -> bool{
        let map = unsafe{ &*self.map };
        map.contains_key(key)
    }
    pub fn len(&self) -> usize{
        let map = unsafe{ &*self.map };
        map.len()
    }
    pub fn is_empty(&self) -> bool {
        let map = unsafe { &*self.map };
        map.is_empty()
    }


    pub fn iter(&self) -> MListCPtrIter<V> {
        let map = unsafe{ &*self.map };
        MListCPtrIter::new(unsafe{ map.citer_unsafe() }, self.list_def, self.root_def)
    }


    pub fn iter_from_last(&self) -> MListCPtrIter<V> {
        let map = unsafe{ &*self.map };
        MListCPtrIter::new(unsafe{ map.citer_from_last_unsafe() }, self.list_def, self.root_def)
    }

    pub fn iter_from_id(&self, id : u64) -> Option<MListCPtrIter<V>> {
        let map = unsafe{ &*self.map };
        unsafe{ map.citer_from_id_unsafe(id) }.map(|iter| MListCPtrIter::new(iter, self.list_def, self.root_def))
    }
}
