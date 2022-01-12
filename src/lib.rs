pub mod first;

mod test {



    #[test]
    fn basics() {
        // Init a new List
        let mut list = crate::first::List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_ne!(list.pop(), Some(1));

    }
}
