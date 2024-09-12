// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str::FromStr,
    sync::mpsc,
    thread,
};

use base_inf::tags::{OwnedWord, Tags, Word};
use capitalize::Capitalize;
use jieba_rs::Jieba;
use petgraph::prelude::DiGraphMap;
use rayon::{
    iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSlice,
    str::ParallelString,
};
use unicode_properties::{GeneralCategoryGroup, UnicodeGeneralCategory};

fn main() {
    let mut jieba = Jieba::new();
    let dict = File::open("dict/dict.txt.big").unwrap();
    let mut dict = BufReader::new(dict);
    jieba.load_dict(&mut dict).unwrap();
    jieba.add_word("色谱龙", Some(100000), Some(Tags::Nz.into()));
    jieba.add_word("螺内酯", Some(100001), Some(Tags::Nz.into()));
    jieba.add_word("补佳乐", Some(100002), Some(Tags::Nz.into()));
    jieba.add_word("苏雨晴", None, Some(Tags::Nr.into()));
    jieba.add_word("小晴", None, Some(Tags::Nr.into()));
    jieba.add_word("冲云", None, Some(Tags::Nr.into()));
    jieba.add_word("方莜莜", None, Some(Tags::Nr.into()));
    jieba.add_word("方宇悠", None, Some(Tags::Nr.into()));
    jieba.add_word("张思凡", None, Some(Tags::Nr.into()));
    jieba.add_word("夕子", None, Some(Tags::Nr.into()));
    jieba.add_word("天赋党", None, Some(Tags::Nz.into()));
    jieba.add_word("家长党", None, Some(Tags::Nz.into()));
    jieba.add_word("药娘", None, Some(Tags::Nz.into()));
    jieba.add_word("安念", None, Some(Tags::Nr.into()));
    jieba.add_word("杨豪伟", None, Some(Tags::Nr.into()));
    jieba.add_word("萝莉控", None, Some(Tags::Nz.into()));
    jieba.add_word("林夕晨", None, Some(Tags::Nr.into()));
    jieba.add_word("莫空", None, Some(Tags::Nr.into()));
    jieba.add_word("莜莜", None, Some(Tags::Nr.into()));
    jieba.add_word("莜莜姐", None, Some(Tags::Nr.into()));
    jieba.add_word("思思姐", None, Some(Tags::Nr.into()));
    jieba.add_word("张思凡", None, Some(Tags::Nr.into()));
    jieba.add_word("胡玉牛", None, Some(Tags::Nr.into()));
    jieba.add_word("阿牛", None, Some(Tags::Nr.into()));
    jieba.add_word("柳韵", None, Some(Tags::Nr.into()));
    jieba.add_word("朱志杰", None, Some(Tags::Nr.into()));
    jieba.add_word("冉空城", None, Some(Tags::Nr.into()));
    jieba.add_word("梅经理", None, Some(Tags::Nr.into()));
    jieba.add_word("徐嫂", None, Some(Tags::Nr.into()));
    jieba.add_word("王海峰", None, Some(Tags::Nr.into()));
    jieba.add_word("陈淑艳", None, Some(Tags::Nr.into()));
    jieba.add_word("天语遥", None, Some(Tags::Nr.into()));
    jieba.add_word("小遥", None, Some(Tags::Nr.into()));
    jieba.add_word("萝莉", None, Some(Tags::Nz.into()));
    jieba.add_word("小红鸡", None, Some(Tags::N.into()));
    jieba.add_word("违和感", None, Some(Tags::N.into()));
    jieba.add_word("素颜", None, Some(Tags::N.into()));
    jieba.add_word("通模具", None, Some(Tags::V.into()));
    jieba.add_word("去势手术", None, Some(Tags::V.into()));
    jieba.add_word("抗雄", None, Some(Tags::V.into()));
    jieba.add_word("闭上", None, Some(Tags::V.into()));
    jieba.add_word("闭着", None, Some(Tags::V.into()));
    jieba.add_word("上衣", None, Some(Tags::N.into()));
    jieba.add_word("高兴", None, Some(Tags::A.into()));
    jieba.add_word("伪声", None, Some(Tags::N.into()));
    jieba.add_word("踏云而去", None, Some(Tags::I.into()));
    jieba.add_word("贼机", Some(100004), Some(Tags::N.into()));
    jieba.add_word("忍住", Some(100003), Some(Tags::V.into()));
    let mut s = "".to_string();
    let _ = File::open("/home/sbchild/transky/src/all.md")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    let s: String = s
        .par_lines()
        .filter(|x| !x.starts_with("#"))
        .map(|x| x.replace(" ", ""))
        .collect();
    let tags = jieba.tag(&s, false);
    let tags: Vec<_> = tags
        .par_iter()
        .map(|x| (Tags::from_str(&x.tag.capitalize()).unwrap(), x.word))
        .map(|x| Word { tag: x.0, w: x.1 })
        .collect();
    let tags: Vec<_> = tags
        .par_split(|x| {
            let c = x.w.chars().next();
            if let Some(c) = c {
                let cg = c.general_category_group();
                if cg == GeneralCategoryGroup::Letter {
                    return false;
                }
            }
            x.tag == Tags::X
        })
        .filter(|x| !x.is_empty())
        .collect();
    let part_of_speech: HashSet<_> = tags
        .par_iter()
        .map(|x| x.par_iter().map(|y| y.tag).collect::<Vec<_>>())
        .collect();
    println!("{:?} {}", tags, tags.len());
    // let flat_tags: Vec<_> = tags.par_iter().flat_map(|r| r.par_iter()).collect();
    let elements_hs: HashSet<_> = tags.par_iter().flat_map(|r| r.par_iter()).collect();
    let elements: Vec<_> = elements_hs
        .par_iter()
        .map(|x| <&Word<'_>>::clone(x).clone())
        .collect();
    // #[derive(Debug)]
    // struct WordNode<'a, Ix> {
    //     word: Word<'a>,
    //     index: NodeIndex<Ix>,
    // }
    // let mut nodes: HashMap<Word<'_>, NodeIndex<_>> = HashMap::new();
    let mut g: DiGraphMap<Word, ()> = DiGraphMap::new();
    for e in elements {
        let _ = g.add_node(e.clone());
        // nodes.insert(e, n);
    }
    // println!("{:?}", nodes);
    let connections: Vec<_> = tags
        .par_iter()
        .map(|x| x.par_windows(2).map(|x| (x[0], x[1])).collect::<Vec<_>>())
        .filter(|x| !x.is_empty())
        .collect();
    // println!("{:?}", connections);
    for c in connections {
        for s in c {
            g.add_edge(s.0, s.1, ());
        }
    }
    // println!("total nodes {}", g.node_count());
    // return;
    // let n: Vec<_> = g
    //     .neighbors_directed(
    //         Word {
    //             w: "苏雨晴",
    //             tag: Tags::Nr,
    //         },
    //         petgraph::Direction::Outgoing,
    //     )
    //     .collect();
    // println!("{:?} {}", n, n.len());
    let (res_chan_tx, res_chan_rx) = mpsc::sync_channel::<Vec<OwnedWord>>(16384 * 32);
    // let (res_chan_tx, res_chan_rx) = mpsc::channel::<Vec<OwnedWord>>();
    thread::spawn(move || {
        let file = File::create("res-2.txt").unwrap();
        let mut file_buf = BufWriter::new(file);
        for i in res_chan_rx.iter().enumerate().map(|x| {
            // println!("nya");
            let words =
                x.1.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
            (x.0, format!("{} {}\n", x.0, words))
        }) {
            // println!("{:?}", i);
            file_buf.write(i.1.as_bytes()).unwrap();
            // if i.0 % (16384 * 16) == 0 {
            //     file_buf.flush().unwrap();
            // }
        }
        file_buf.flush().unwrap();

        // let res: Vec<_> = res_chan_rx.iter().collect();
        // for i in &res {
        //     println!("{:?}", i);
        // }
        // println!("total result {}", res.len());
    });
    let nodes: Vec<_> = g.nodes().collect();
    // for n in nodes {
    // let nb: Vec<_> = g
    //     .neighbors_directed(n, petgraph::Direction::Outgoing)
    //     .collect();
    // println!("current: {}", n);
    nodes
        .par_iter()
        .map(|x| {
            recursion_search(RecursionSearchOpts {
                min_length: 5 as usize,
                max_length: 20 as usize,
                current_word: *x,
                current_conn: vec![*x],
                graph: &g,
                chan: res_chan_tx.clone(),
                rules: &part_of_speech,
            });
            println!("{} done", x);
        })
        .collect::<Vec<_>>();
    // }
    println!("total nodes {}", g.node_count());
}

struct RecursionSearchOpts<'a> {
    min_length: usize,
    max_length: usize,
    current_word: Word<'a>,
    current_conn: Vec<Word<'a>>,
    chan: mpsc::SyncSender<Vec<OwnedWord>>,
    // chan: mpsc::Sender<Vec<OwnedWord>>,
    rules: &'a HashSet<Vec<Tags>>,
    graph: &'a DiGraphMap<Word<'a>, ()>,
}

fn recursion_search(opts: RecursionSearchOpts) {
    if opts.current_conn.len() + 1 > opts.max_length {
        return;
    }

    let nb: Vec<_> = opts
        .graph
        .neighbors_directed(opts.current_word, petgraph::Direction::Outgoing)
        // 组合不可以使用重复单词
        .filter(|x| *x != opts.current_word)
        .filter(|x| !opts.current_conn.iter().any(|y| x == y))
        .collect();
    // 最后一个词的词性
    let word_tags: Vec<_> = nb.par_iter().map(|x| x.tag).collect();
    // 当前组合的词性
    let conn_tags: Vec<_> = opts.current_conn.par_iter().map(|x| x.tag).collect();
    let matched_rules: Vec<_> = opts
        .rules
        .par_iter()
        // 长度相等
        .filter(|x| x.len() == opts.current_conn.len() + 1)
        // 最后一个词的词性相同
        .filter(|x| word_tags.par_iter().any(|y| x.last() == Some(y)))
        // 前面部分的词性匹配当前组合
        .filter(|x| {
            x.par_iter()
                .take(conn_tags.len())
                .zip(conn_tags.par_iter())
                .all(|y| y.0 == y.1)
        })
        // 不为空
        .filter(|x| !x.is_empty())
        // 取结尾部分
        .map(|x| x.last().unwrap())
        .collect();
    if matched_rules.len() <= 0 {
        return;
    }
    for w in nb
        .into_iter()
        .filter(|x| matched_rules.par_iter().any(|y| **y == x.tag))
    {
        let mut current = opts.current_conn.clone();
        current.push(w);
        // let pos: Vec<_> = current.par_iter().map(|x| x.tag).collect();
        // println!("- {:?}", current);
        if current.len() >= opts.min_length && current.len() <= opts.max_length {
            let c: Vec<_> = current.iter().map(|x| x.owned()).collect();
            opts.chan.send(c).unwrap();
        }
        if current.len() < opts.max_length {
            recursion_search(RecursionSearchOpts {
                min_length: opts.min_length,
                max_length: opts.max_length,
                current_word: w,
                current_conn: current,
                graph: opts.graph,
                chan: opts.chan.clone(),
                rules: opts.rules,
            });
        }
    }
}
