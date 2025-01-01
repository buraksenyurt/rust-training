use std::io::{self, BufRead, Write};

/*
    stdin, stdout ve hatta stderr UNIX tabanlı sistemlerden beri gelen birer standardı ifade eder.
    stdin temelde kullanıcıdan veya başka bir programdan veri almak,
    stdout programın normal çıktısını kullanıcıya veya başka bir programa iletmek,
    stderr ise hata mesajlarını ayırmak maksadıyla geliştirilmiş standartlardır.

    Bu noktada |(pipe) ve >(redirect) sıklıkla karşılaşılan terminal operatörleridir.
    |, stdout çıktısını başka bir programın stdin girişine yönlendirmek için kullanılır.

    Örneğin,
        ls | grep ".rs"
    gibi.

    > ise bir komutun stdout çıktısını bir dosyaya yönlendirmek için kullanılır.
    Örneğin,
        ls > files.txt
    gibi.

    Birde >>(Append) vardır. > gibi yönlendirme yapar ama mevcut dosyasının üzerine yazmak yerine
    sonuna ekleme yapar.
    Örneğin,
        echo "Some thoughts about you" >> memories.txt

    stdin ve stdout ile stream'ler üzerinden veri yazmak veya okumak mümkündür.
    Örneğin bir dosyaya stdin ile veriyi write_to_file metodundaki gibi yazabiliriz.
    Ya da terminalden gelen verileri okumak için stdin().read_line metodu kullanılabilir (sum örneği)

    Kullanıcı terminalden bilgi girdikçe de işlenmesini sağlayabiliriz. Bir stdin açıp
    satır satır okumak yeterlidir(read metodu)

    Hatta pipe üzerinden gelen bir stream' de ele alınabilir. (read_from_pipe metodu)

    Rust'ta stdout başka bir programın girdisine stdin üzerinden bağlanarak veri aktarabilir.
    Örneğin bir programın çalışma zamanı çıktısını başka bir dosyaya yazarken stdout kullanılabilir.

*/
fn write_to_file() -> io::Result<()> {
    let mut input = String::new();
    println!("Please enter some text:");

    io::stdin().read_line(&mut input)?;
    println!("Your text is: {}", input.trim());

    Ok(())
}

fn sum() -> io::Result<i32> {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Please enter the first number:");
    io::stdin().read_line(&mut input1)?;

    println!("Second number:");
    io::stdin().read_line(&mut input2)?;

    let x: i32 = input1.trim().parse().expect("Please enter a number!");
    let y: i32 = input2.trim().parse().expect("Please enter a number!");

    Ok(x + y)
}

fn read() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Please enter some text (Ctrl+Z for exit):");
    for line in reader.lines() {
        let line = line?;
        println!("Input: {}", line);
    }

    Ok(())
}

/*

Aşağıdaki metod için terminalden şu komut verilebilir.

cat logs.dat | cargo run
*/

fn read_from_pipe() -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    println!("Data is retrieving...");
    for line in reader.lines() {
        let line = line?;
        println!("Data: {}", line);
    }

    Ok(())
}

/*
    Aşağıdaki metodu işletmek için programı şu şekilde işletebiliriz.

    cargo run > logs.txt

    Burada kullanılan > operatörü, programdaki stdout'u logs.txt dosyasına doğru yönlendirir.
*/
fn write_log() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "This will be written to a file.")?;

    Ok(())
}

/*
    Aşağıdaki metod verilen teriman komutuna göre şöyle çalışır.
    logs.dat dosyası cat uygulaması ile açılır, bu uygulama dosyanın içeriğini okur ve
    pipe vasıtasıyla bu programın stdin girdisine aktarılır. Bu programda gelen içeriği alır
    ve onu output_logs.txt dosyasının içeriğine doğru yönlendirir.

    cat logs.dat | cargo run > output_logs.txt

*/
fn double_usage() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let reader = stdin.lock();
    let mut writer = stdout.lock();

    for line in reader.lines() {
        let line = line?;
        writeln!(writer, "Data received {}", line)?;
    }
    Ok(())
}

pub fn run() -> io::Result<()> {
    // write_to_file()?;
    //
    // let total = sum()?;
    // println!("Total: {}", total);
    //
    // read()?;

    // read_from_pipe()?;

    // write_log()?;

    double_usage()?;

    Ok(())
}
