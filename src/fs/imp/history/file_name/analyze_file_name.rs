use crate::fs::imp::history::file_name::file_name_props::FileNameProps;

/// #hogehoge#_0_0_0_0_0_0.his といった感じである。tagがない場合
/// _0_0_0_0_0_0.his となる。
///
/// 最初の数字は管理番号であり、newされると1増える。newしたら管理番号_0にリセットされる。
///
/// それ以外は、next時に増えていく。
///
/// 最初は管理番号 0 の 0 なので 0_0
/// next すると 0_0_0
/// 簡単のため、仮に2-phaseとすると、次にnextしたとき 0_0_1になる。
/// そして 0_0_2・・・と増えていくが条件を満たすとphase shiftする。phase Aを1増やす
///
/// 0_1
///
/// そしてnextでまた増えていく
///
/// 0_1_0
/// 0_1_1 ...
///
/// 0_1_0 や 0_1_1 を開くためには、元となる 0_1 が必要である。そういった依存関係がファイル名として記されていて、
/// ファイル名をいじるとこのシステムはぶっ壊れる
///
/// で、newすると
///
/// 1_0 となる。
///
/// 管理番号は実際なくてもよく、Phase A の値を増やせば実務上はいいのだが、ちょっと分かりにくい気がするので管理番号も用意する
/// （それどころか、殆どの場合においてnewでなくnextし続けたほうが効率的だと思う・・・)
///
/// 実装には正規表現を使うべきだろうが、手で書いたほうが動作速度が速いと思っているし、この処理はできるだけ速くしたい
pub(crate) fn analyze_file_name(s : &str, hint_max_phase : Option<usize>) -> Option<FileNameProps>{
    if s.ends_with(".his") == false{
        return None;
    }

    let tag = get_tag(s);

    let mut s = if let Some(tag) = tag{
        &s[(tag.len() + 2)..]
    } else{
        s
    };

    let n = if let Some((num, read)) = get_num(s.as_bytes()){
        s = &s[read..];
        num
    } else{
        return None;
    };

    let capacity = hint_max_phase.unwrap_or(0) + 1;
    let mut order : Vec<u32> = Vec::with_capacity(capacity);
    loop{
        if let Some((num, read)) = get_num(s.as_bytes()){
            order.push(num);
            s = &s[read..];
        } else{
            if s == ".his" && order.len() != 0{
                if let Ok(props) = FileNameProps::new(n, order, tag.map(|t| t.to_string())){
                    return Some(props);
                }
            }
            return None;
        }
    }
}

fn get_tag(s : &str) -> Option<&str>{
    let bytes = s.as_bytes();
    if bytes.len() <= 1{ return None; }
    let first = bytes[0];
    if first != '#' as u8{
        return None;
    }
    for (i,c) in bytes.iter().skip(1).enumerate(){
        if *c == '#' as u8{
            return Some(&s[1..i])
        }
    }
    return None;
}

///numと読んだ文字数を返す
fn get_num(s : &[u8]) -> Option<(u32, usize)>{
    if s.len() == 0 { return None; }
    if s[0] != '_' as u8{ return None; }
    let mut i = 1;
    let mut result : u32 = 0;

    loop{
        if s.len() <= i{ return None; }

        let b = s[i];
        if '0' as u8 <= b && b <= '9' as u8{
            result *= 10;
            result += (b - '0' as u8) as u32;
        } else{
            return Some((result, i));
        }
        i += 1;
        if i == 10{ return None; } //オーバーフローは避ける
    }


}