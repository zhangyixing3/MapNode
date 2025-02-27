use bstr::{io::BufReadExt, ByteSlice};
use core::panic;
use fxhash::FxHashMap;
use fxhash::FxHashSet;
use std::{env, fs::File, io::BufReader, path::Path};

fn parse_s_lines<P: AsRef<std::path::Path>>(path: P) -> FxHashMap<u64, u64> {
    let mut node_lengths = FxHashMap::default();
    let file = File::open(path).unwrap(); // 使用unwrap处理错误
    let reader = BufReader::new(file);

    for line in reader.byte_lines() {
        let line = line.unwrap(); // 使用unwrap处理错误
        if line.starts_with(b"S") {
            let fields: Vec<&[u8]> = line.split(|&c| c == b'\t').collect();
            // 解析节点ID为u64（直接panic错误）
            let node_id = parse_u64(fields[1]);
            let length = fields[2].len() as u64;
            node_lengths.insert(node_id, length);
        }
    }
    node_lengths
}
fn generate_accumulated_lengths<P: AsRef<Path>>(
    path: P,
    node_lengths: &FxHashMap<u64, u64>,
) -> FxHashMap<String, FxHashMap<String, (Vec<u64>, Vec<u64>)>> {
    let mut result: FxHashMap<String, FxHashMap<String, (Vec<u64>, Vec<u64>)>> =
        FxHashMap::default();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.byte_lines() {
        let line = line.unwrap();
        if line.starts_with(b"W") {
            let mut fields = line.split_str("\t");
            // 解析关键字段
            let sample = fields.nth(1).unwrap().to_str().unwrap();
            let chrom = fields.nth(1).unwrap().to_str().unwrap();
            let start = parse_u64(fields.next().unwrap());
            let end = parse_u64(fields.next().unwrap());
            let nodes = fields.next().unwrap();

            // 解析节点序列（保留原始ID）
            let node_sequence: Vec<u64> = nodes
                .split(|&c| c == b'>' || c == b'<')
                .filter(|s| !s.is_empty())
                .map(parse_u64)
                .collect();

            // 生成累加和
            let mut accum = vec![start];
            let mut sum = start;
            for id in &node_sequence {
                sum += node_lengths.get(id).expect("找不到节点长度");
                accum.push(sum);
            }

            assert_eq!(
                sum, end,
                "累加和校验失败: sample={}, chrom={}, start={}, end={}",
                sample, chrom, start, end
            );
            assert!(accum.len() == node_sequence.len() + 1);

            // 插入结果
            result
                .entry(sample.to_owned())
                .or_default()
                .insert(chrom.to_owned(), (accum, node_sequence));
        }
    }

    result
}
/// 高性能字节到u64的解析（错误时直接panic）
pub fn parse_u64(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0u64, |acc, &b| {
        if b.is_ascii_digit() {
            acc * 10 + (b - b'0') as u64
        } else {
            panic!("Invalid byte in u64 parsing: {}", b);
        }
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <raw.gfa> <sub.gfa>", args[0]);
        std::process::exit(1);
    }

    // 解析原始GFA数据
    let raw_node_lengths = parse_s_lines(&args[1]);
    let raw_accum_data = generate_accumulated_lengths(&args[1], &raw_node_lengths);

    // 处理子图文件
    let sub_file = File::open(&args[2]).unwrap();
    let sub_node_lengths = parse_s_lines(&args[2]);
    let sub_reader = BufReader::new(sub_file);
    let mut out = FxHashSet::default();


    for line in sub_reader.byte_lines() {
        let line = line.unwrap();
        if line.starts_with(b"W") {
            let mut fields = line.split_str("\t");

            // 解析子图路径信息
            let sample = fields.nth(1).unwrap().to_str().unwrap();
            let chrom = fields.nth(1).unwrap().to_str().unwrap();
            let sub_start = parse_u64(fields.next().unwrap());
            let _sub_end = parse_u64(fields.next().unwrap());
            let sub_nodes = fields.next().unwrap();

            // 获取原始路径数据
            let Some((raw_accum, raw_nodes)) =
                raw_accum_data.get(sample).and_then(|m| m.get(chrom))
            else {
                eprintln!("原始路径不存在: {}/{}", sample, chrom);
                continue;
            };
            // 遍历子图节点
            let mut current_sub_pos = sub_start;
            for sub_node in sub_nodes
                .split(|&c| c == b'>' || c == b'<')
                .filter(|s| !s.is_empty())
            {
                let sub_node_id = parse_u64(sub_node);

                let sub_node_len = sub_node_lengths[&sub_node_id];

                let start_idx = match raw_accum.binary_search(&current_sub_pos) {
                    Ok(i) => i,
                    Err(i) => i.saturating_sub(1),
                };

                current_sub_pos += sub_node_len;
                if out.contains(&sub_node_id) {
                    continue;
                }else {
                    out.insert(sub_node_id);
                    println!("{}\t{}", sub_node_id, raw_nodes[start_idx]);
                }



            }


        }
    }
}
