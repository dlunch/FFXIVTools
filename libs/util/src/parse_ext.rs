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
macro_rules! cast {
    ($data: expr, $type: ty) => {
        LayoutVerified::<&[u8], $type>::new(&$data[..core::mem::size_of::<$type>()]).unwrap()
    };
}
