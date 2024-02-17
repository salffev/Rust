use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u64,
    description: String,
    completed: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    // Görev ekleme
    tasks.push(Task {
        id: 1,
        description: "Alışveriş yapmak".to_string(),
        completed: false,
    });

    // Görevleri listeleme
    for task in &tasks {
        println!("ID: {}", task.id);
        println!("Açıklama: {}", task.description);
        println!("Tamamlandı: {}", if task.completed { "Evet" } else { "Hayır" });
        println!();
    }

    // Görev silme
    tasks.remove(0);

    // Görevleri tekrar listeleme
    for task in &tasks {
        println!("ID: {}", task.id);
        println!("Açıklama: {}", task.description);
        println!("Tamamlandı: {}", if task.completed { "Evet" } else { "Hayır" });
        println!();
    }
}
