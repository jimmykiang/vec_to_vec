#[derive(Debug, Clone)]
struct Structure<T> {
    m_1: T,
}

fn main() {
    let mut vec_1 = Vec::<Structure<String>>::new();
    vec_1.push(Structure {
        m_1: "a".to_string(),
    });
    vec_1.push(Structure {
        m_1: "b".to_string(),
    });
    vec_1.push(Structure {
        m_1: "c".to_string(),
    });

    println!("vec_1: {:?}", vec_1);

    let mut vec_2 = Vec::<Structure<String>>::new();

    vec_2 = vec_1.to_vec();

    println!("vec_2: {:?}", vec_2);
}
