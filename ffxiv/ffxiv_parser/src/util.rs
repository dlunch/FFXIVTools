macro_rules! parse {
    ($data: expr, $type: ty) => {
        <$type>::parse(&$data).unwrap().1
    };

    ($data: expr, $cursor: expr, $type: ty) => {{
        let result = parse!(&$data[$cursor..], $type);
        $cursor += <$type>::SIZE;

        result
    }};

    ($data: expr, $cursor: expr, $count: expr, $type: ty) => {
        (0..$count).map(|_| parse!($data, $cursor, $type)).collect::<Vec<_>>()
    };
}
