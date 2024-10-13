fn main() {
    /*
       Vector türü heap üstünden yer allocate etmeye izin verir ve dinamik olarak büyüyebilir.
       Diziler gibi sabit elemanlı değillerdir.
       Dizilerde olduğu gibi aynı türden verileri taşıyabilirler.
    */

    let mut points = vec![3.14, 2.67, 1.24, 9.80];
    points.push(7.5);
    println!("{:?}", points);
    println!("First point is {}", points[0]);

    let last_in = points.pop();
    println!("{:?}", last_in);
    println!("Current vector elements {:#?}", points);

    let mut names = Vec::new();
    names.push(String::from("Bob"));
    names.push(String::from("Frank"));
    names.push(String::from("Orange"));
    names.push(String::from("Mary"));
    names.reverse();
    println!("Players vector capacity is {:?}", names.capacity());
    println!("Players :\n {:#?}", names);

    let codes: Vec<u8> = (50..=60).collect();
    println!("Looking codes are, {:?}", codes);

    /*
       Vector/Array türlerinin işaret ettiği veri kümelerinden dilimler alabiliriz.
       Yani tüm veri kümesini almak yerine sadece belli bir dilimi ile çalışma yapmak istediğimizde,
       bu dilime ait bir referans ile çalışmak tercih edilir.
       Slice olarak geçen bir kavram aslında bir veri dilimini referans eden bir değişkendir.
    */
    let codes: Vec<u8> = (0..100).collect();
    let first_ten = &codes[0..=10];
    println!("First ten: {:#?}", first_ten);
}
