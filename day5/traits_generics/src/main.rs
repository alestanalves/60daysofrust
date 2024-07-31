// Definindo um trait chamado Summary
trait Summary {
    // Declaração do método summarize que deve ser implementado por qualquer tipo que implemente este trait
    fn summarize(&self) -> String;
}

// Definindo a struct NewsArticle
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Implementando o trait Summary para a struct NewsArticle
impl Summary for NewsArticle {
    // Implementação do método summarize para NewsArticle
    fn summarize(&self) -> String {
        // Retorna uma string formatada com os campos da struct
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Definindo a struct Tweet
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Implementando o trait Summary para a struct Tweet
impl Summary for Tweet {
    // Implementação do método summarize para Tweet
    fn summarize(&self) -> String {
        // Retorna uma string formatada com os campos da struct
        format!("{}: {}", self.username, self.content)
    }
}

// Função genérica que aceita qualquer tipo que implemente o trait Summary
fn notify(item: &impl Summary) {
    // Imprime uma mensagem de notificação chamando o método summarize do item
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    // Cria uma instância de NewsArticle
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Jane Doe"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    // Cria uma instância de Tweet
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("The world is a big place."),
        reply: false,
        retweet: false,
    };

    // Chama a função notify com uma referência a article
    notify(&article);
    // Chama a função notify com uma referência a tweet
    notify(&tweet);
}
