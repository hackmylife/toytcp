#[cfg(test)]
mod packet;
mod socket;
pub mod tcp;
mod tcpflags;

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
