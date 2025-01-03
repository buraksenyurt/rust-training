/*
    Aynı türden nesneleri çok sık oluşturup(create) kaldırdığımız(drop) bir durum olduğunu ve bunu
    kısıtlı bellek kapasitesine sahip bir gömülü sistem üzerinde işletmemiz gerektiğini düşünelim.
    Böyle bir durumda kendi bellek havuzumu oluşturup yönetebiliriz ama bunun için unsafe alana
    çıkmamız gerekir. Aşağıdaki kodlarda bu kullanima ait bir örnek yer alıyor.

    Olası hataları ise şöyle ifade edebiliriz.

    User After Free: Serbest kalan bellek bloğuna erişmeye çalışmak
    Double Free : Aynı bellek bloğunu birden fazla kez serbest bırakılmaya çalışılması
*/
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;

// const X64_ALIGN: usize = 8;

/*
   Sistemin 32 veya 64 bit olmasına göre gerekli olan byte hizalama değerini aldığımı fonksiyon
*/
fn get_alignment() -> usize {
    if cfg!(target_pointer_width = "32") {
        4 // 32-bit alignment
    } else {
        8 // 64-bit alignment
    }
}

struct RapidMemoryPool {
    total_size: usize,
    usage: usize,
    ref_pointer: *mut u8, //raw memory pointer
}

impl RapidMemoryPool {
    fn new(total_size: usize) -> Self {
        println!("Rapid Memory Pool initiating");

        unsafe {
            let layout = Layout::from_size_align(total_size, get_alignment()).unwrap();
            println!("{layout:?}");
            let ref_pointer = alloc(layout);
            if ref_pointer.is_null() {
                panic!("Could not allocate memory");
            }
            Self {
                total_size,
                usage: 0,
                ref_pointer,
            }
        }
    }

    /*
        Bu fonksiyon bir bellek bölgesini ayırır ve başlangıç adresini
        raw pointer olarak döner. Buradaki raw pointer mutable'dır.
    */
    fn allocate(&mut self, amount: usize) -> *mut u8 {
        unsafe {
            if self.usage + amount > self.total_size {
                panic!("Out of memory");
            }
            let alloc_ptr = self.ref_pointer.add(self.usage);
            self.usage += amount;
            alloc_ptr
        }
    }

    fn free(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(self.total_size, get_alignment()).unwrap();
            dealloc(self.ref_pointer, layout);
            self.ref_pointer = ptr::null_mut();
            self.total_size = 0;
            self.usage = 0;
        }
    }
}

impl Drop for RapidMemoryPool {
    fn drop(&mut self) {
        if !self.ref_pointer.is_null() {
            self.free();
            println!("Dropping memory pool");
        }
    }
}

pub fn run() {
    let mut pool = RapidMemoryPool::new(1024);

    unsafe {
        let block_red = pool.allocate(256); // 256 byte yer ayırır
        println!("Block Red allocated at: {:?}", block_red);

        let block_blue = pool.allocate(512); // 512 byte yer ayırır
        println!("Block Blue allocated at: {:?}", block_blue);
    }
} // Drop trait implementasyonu nedeniyle buradan bellek otomatik serbest kalır