fn main() {
    let mut val: u32 = 1;
    let mut found : bool = false;
    while !found {
    found = true;
        for i in 1..20{
            if val % i !=0{
                found=false;
                val+=1;
                break;
            }
        }
    }
    print!("{}", val);
    return;
}

