fn main() {
    let v = "hello world";
    assert_eq!(v,"hello world");
    {
        let v = "hello rust";
        assert_eq!(v,"hello rust");

    }
    assert_eq!(v,"hello world");
    let a = 10;
    let b = 20;
    let r = doSome(sum,a,b);
    assert_eq!(r,30)

}

fn sum(a:usize,b:usize) -> usize{
    a+b
}
fn doSome(op: fn(usize,usize) -> usize,a:usize,b:usize)->usize {
    op(a,b)
}