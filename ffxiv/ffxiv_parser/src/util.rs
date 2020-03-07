macro_rules! parse {
    ($data: expr, $offset: expr, $type: ty) => {{
        $data.advance($offset);
        let result = <$type>::parse(&$data).unwrap().1;
        $data.advance(<$type>::SIZE);

        result
    }};
}
