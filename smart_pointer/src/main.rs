fn main() {
    let boxed = Box::new(5);
    let unboxed = 5;

    assert_eq!(*boxed, unboxed);

    trait Animal {}

    struct Cat {}
    impl Animal for Cat {}

    struct Dog {}
    impl Animal for Dog {}

    let cat: Box<dyn Animal> = Box::new(Cat {});
    let dog: Box<dyn Animal> = Box::new(Dog {});

    // async fn async_fn() {}
    // let pinned = Box::pin(async_fn()).await;

    let raw_cat_pointer = Box::into_raw(cat);
    let new_cat = unsafe { Box::from_raw(raw_cat_pointer) };
}
