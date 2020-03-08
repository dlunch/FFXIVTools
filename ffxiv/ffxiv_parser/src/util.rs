macro_rules! parse {
    ($data: expr, $type: ty) => {{
        let result = <$type>::parse(&$data).unwrap().1;
        $data.advance(<$type>::SIZE);

        result
    }};

    ($data: expr, $count: expr, $type: ty) => {{
        (0..$count).map(|_| parse!($data, $type)).collect::<Vec<_>>()
    }};
}
