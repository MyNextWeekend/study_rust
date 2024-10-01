use std::sync::{Arc, Mutex};

trait Image {
    fn display(&self);
}

// 实际图像类
struct RealImage {
    filename: String,
}

impl RealImage {
    fn new(filename: &str) -> Self {
        println!("Loading image: {}", filename);
        RealImage {
            filename: filename.to_string(),
        }
    }
}

impl Image for RealImage {
    fn display(&self) {
        println!("Displaying image: {}", self.filename);
    }
}

// 代理图像类
struct ProxyImage {
    real_image: Arc<Mutex<Option<RealImage>>>,
    filename: String,
}

impl ProxyImage {
    fn new(filename: &str) -> Self {
        ProxyImage {
            real_image: Arc::new(Mutex::new(None)),
            filename: filename.to_string(),
        }
    }
}

impl Image for ProxyImage {
    fn display(&self) {
        // 懒加载
        let mut real_image = self.real_image.lock().unwrap();
        if real_image.is_none() {
            *real_image = Some(RealImage::new(&self.filename));
        }
        real_image.as_ref().unwrap().display();
    }
}

// 客户端
fn main() {
    let image = Arc::new(ProxyImage::new("test_image.jpg"));

    let image_clone = Arc::clone(&image);
    std::thread::spawn(move || {
        image_clone.display(); // 第一次显示会加载图像
    }).join().unwrap();

    image.display(); // 第二次显示不再加载图像
}



#[cfg(test)]
mod proxy_test {
    use super::*;


    #[test]
    fn test_01() {
        // 客户端
        let image: Box<dyn Image> = Box::new(ProxyImage::new("test_image.jpg"));

        // 第一次显示会加载图像
        image.display();

        // 第二次显示不再加载图像
        image.display();
    }
}

