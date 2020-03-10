macro_rules! parse {
    ($data: expr, $type: ty) => {
        <$type>::parse(&$data).unwrap().1
    };
    ($data: expr, $count: expr, $type: ty) => {
        (0..$count).map(|x| parse!(&$data[x * <$type>::SIZE..], $type)).collect::<Vec<_>>()
    };
}
