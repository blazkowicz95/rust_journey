fn linear_search(value:i32, _arr:&[i32])->i32{
    let result:i32 = -1;
    let mut count:i32 = 0;
    for elem in 0.._arr.len(){
        count+=1;
        if value == _arr[elem]{
            return count;
        }
    }
    return result;
    
}

fn main(){
    let xs:[i32; 5] = [1,2,3,4,5];
    let res:i32 = linear_search(1, &xs);
    println!("{}",res)
}