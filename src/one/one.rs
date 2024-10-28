pub trait One {
    const ONE: Self; // TODO Should be a function so that it works with types that can't be created
                     // at comptime
}
