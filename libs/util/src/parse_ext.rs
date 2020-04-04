#[macro_export]
macro_rules! parse {
    ($data: expr, $type: ty) => {
        <$type>::parse(&$data).unwrap().1
    };
    ($data: expr, $count: expr, $type: ty) => {
        (0..$count as usize)
            .map(|x| $crate::parse!(&$data[x * <$type>::SIZE..], $type))
            .collect::<Vec<_>>()
    };
}

#[macro_export]
macro_rules! read_and_parse {
    ($file: expr, $offset: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, <$type>::SIZE as usize).await?;
            Ok::<_, std::io::Error>($crate::parse!(data, $type))
        }
    };

    ($file: expr, $offset: expr, $count: expr, $type: ty) => {
        async {
            let data = $file.read_bytes($offset as u64, $count as usize * <$type>::SIZE).await?;
            Ok::<_, std::io::Error>($crate::parse!(data, $count, $type))
        }
    };
}
