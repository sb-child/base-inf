// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.


use std::fmt::{self, Debug, Display, Formatter};

// sources:
// https://github.com/fxsjy/jieba/blob/master/README.md
// https://blog.csdn.net/enter89/article/details/80619805
// https://github.com/duoergun0729/nlp/blob/master/%E4%BD%BF%E7%94%A8Jieba%E8%BF%9B%E8%A1%8C%E4%B8%AD%E6%96%87%E8%AF%8D%E6%80%A7%E6%A0%87%E6%B3%A8.md
// https://gist.github.com/hscspring/c985355e0814f01437eaf8fd55fd7998
// https://github.com/brynne8/jieba/blob/master/README.md
#[derive(
    strum::EnumString,
    strum::IntoStaticStr,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Copy,
)]
pub enum Tags {
    /// 时间
    T,
    /// 时态助词
    Ug,
    /// 动语素 (动词性语素。动词代码为 v。在语素的代码g前面置以V。)
    Vg,
    /// 地名
    Ns,
    /// 副语素 (副词性语素。副词代码为 d，语素代码ｇ前面置以D。)
    Dg,
    /// 副词
    D,
    /// 形容词
    A,
    /// 代词
    R,
    /// 区别词
    B,
    /// 方位名词
    F,
    /// 机构名
    Nt,
    /// 助词
    U,
    /// 时语素 (时间词性语素。时间词代码为 t,在语素的代码g前面置以T。)
    Tg,
    /// 其他专名
    Nz,
    /// -
    Df,
    /// 状态语素
    Zg,
    /// 不及物动词
    Vi,
    /// 指示代词
    Rz,
    /// 人称代词
    Rr,
    /// 代词性语素
    Rg,
    /// 非语素字 (非语素字只是一个符号，字母 x通常用于代表未知数、符号。)
    X,
    /// 普通名词
    N,
    /// 简称略语
    J,
    /// 名语素 (名词性语素。名词代码为 n，语素代码ｇ前面置以N。)
    Ng,
    /// 习用语 (习用语尚未成为成语，有点“临时性”，取“临”的声母。)
    L,
    /// 拟声词
    O,
    /// 动副词
    Vd,
    /// 结构助词 (得)
    Ud,
    /// 名动词
    Vn,
    /// 叹词
    E,
    /// 古代人名
    Nrfg,
    /// 后接成分
    K,
    /// 时态助词 (着)
    Uz,
    /// 人名
    Nr,
    /// 介词
    P,
    /// 副形词
    Ad,
    /// 语气词
    Y,
    /// 名形词
    An,
    /// 结构助词 (的)
    Uj,
    /// 动词
    Vq,
    /// 量词
    Q,
    /// 数量词
    M,
    /// 前接成分
    H,
    /// 成语
    I,
    /// 结构助词 (地)
    Uv,
    /// 数语素
    Mg,
    /// 状态词
    Z,
    /// 连词
    C,
    /// 语素 (绝大多数语素都能作为合成词的“词根”，取汉字“根”的声母。)
    G,
    /// 形语素 (形容词性语素。形容词代码为 a，语素代码ｇ前面置以A。)
    Ag,
    /// 处所名词
    S,
    /// 普通动词
    V,
    /// 时态助词
    Ul,
    /// 音译人名
    Nrt,
    /// 数量词
    Mq,
    /// 英文
    Eng,
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct Word<'a> {
    pub w: &'a str,
    pub tag: Tags,
}

impl Word<'_> {
    pub fn owned(&self) -> OwnedWord {
        OwnedWord {
            w: self.w.to_string(),
            tag: self.tag,
        }
    }
}

impl Display for Word<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tag_str: &str = self.tag.clone().into();
        write!(f, "{}/{}", self.w, tag_str)
    }
}

impl Debug for Word<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Word")
        //     .field("w", &self.w)
        //     .field("tag", &self.tag)
        //     .finish()
        let tag_str: &str = self.tag.clone().into();
        write!(f, "{}/{}", self.w, tag_str)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OwnedWord {
    pub w: String,
    pub tag: Tags,
}

impl Display for OwnedWord {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let tag_str: &str = self.tag.clone().into();
        write!(f, "{}/{}", self.w, tag_str)
    }
}

impl Debug for OwnedWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tag_str: &str = self.tag.clone().into();
        write!(f, "{}/{}", self.w, tag_str)
    }
}

