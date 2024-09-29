use std::path::PathBuf;

fn main() {
    // 指定したディレクトリを再帰的に検索し、proto ファイルを探す
    let proto_dir = vec!["proto"];
    let protos: Vec<_> = proto_dir.iter().fold(vec![], |mut acc, base| {
        acc.append(&mut find_protos(base));
        acc
    });

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&protos, &vec![""])
        .unwrap();
}

fn find_protos(base: &str) -> Vec<PathBuf> {
    let mut protos = Vec::new();
    for entry in walkdir::WalkDir::new(base).into_iter() {
        let entry = entry.unwrap();

        // Directory はスキップ
        if entry.path().is_dir() {
            continue;
        }

        // 拡張子が proto であるならば返り値に追加する
        let path = entry.into_path();
        if path.extension().unwrap() == "proto" {
            protos.push(path)
        }
    }
    protos
}
