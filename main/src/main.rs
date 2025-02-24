// Filtreleme koşulunu tutan struct
struct FilterCondition {
    value: i32,
}

// FilterCondition için metodlar
impl FilterCondition {
    // Bir sayının filtreleme koşuluna uyup uymadığını kontrol eden metod
    fn is_match(&self, item: &i32) -> bool {
        item > &self.value  // Örnek: sayı filter değerinden büyükse true döner
    }
}

// Filtreleme işlemini yapan fonksiyon
fn custom_filter(collection: &Vec<i32>, condition: &FilterCondition) -> Vec<i32> {
    let mut filtered = Vec::new();
    
    for item in collection {
        if condition.is_match(item) {
            filtered.push(*item);
        }
    }
    
    filtered
}

fn main() {
    // Test için bir vektör oluştur
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Filtreleme koşulunu oluştur (5'ten büyük sayıları filtrele)
    let condition = FilterCondition { value: 5 };
    
    // Filtreleme işlemini gerçekleştir
    let filtered_numbers = custom_filter(&numbers, &condition);
    
    // Sonuçları yazdır
    println!("Orijinal sayılar: {:?}", numbers);
    println!("5'ten büyük sayılar: {:?}", filtered_numbers);
} 