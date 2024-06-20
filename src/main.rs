fn main()
{
    let mut n = String::new();
    let fibonacciarray: [u32; 3] = [0, 1, 1];

    println!("Enter the value of n in the Fibonacci Sequence which's value you want to know.");
    std::io::stdin().read_line(&mut n).expect("Error occurred.");
    let n: u32 = n.trim().parse().expect("Please type a number.");

    nthfibonacci(fibonacciarray[0], fibonacciarray[1], fibonacciarray[2], n);
}

fn nthfibonacci(x:u32, y:u32, z:u32, n: u32)
{
    let mut counter: u32 = 0;
    let mut localarray: [u32; 3] = [x, y, z];
    while counter != n
    {
    for i in 0..2
        {
        if i == 0
            {
            localarray[i + 2] = localarray[i + 1] + localarray[i];
            counter += 1;
            }
        else if i == 1
            {
            localarray[i + 1] = localarray[i] + localarray[i + 1];
            counter += 1;
            }
        else if i == 2
            {
            localarray[i - 2] = localarray[i] + localarray[i - 1];
            counter += 1;
            }
        }
    }
    let index: usize = (n % 3).try_into().unwrap();
    println!{"{}", localarray[index]}
}
