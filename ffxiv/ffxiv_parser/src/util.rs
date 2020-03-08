macro_rules! parse {
    ($data: expr, $type: ty) => {{
        let result = <$type>::parse(&$data).unwrap().1;
        $data.advance(<$type>::SIZE);

        result
    }};

    ($data: expr, $count: expr, $type: ty) => {{
        let mut result = Vec::with_capacity($count);
        for _ in 0..$count {
            result.push(<$type>::parse(&$data).unwrap().1);
            $data.advance(<$type>::SIZE);
        }

        result
    }};
}
