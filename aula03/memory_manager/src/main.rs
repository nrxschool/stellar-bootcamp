//! # Gerenciamento de Memória em Rust
//! **Estes exemplos demonstram conceitos de gerenciamento de memória em Rust incluindo:**
//!
//! - [Ownership e Borrowing](fn.main.html#1-ownership-e-borrowing)
//! - [Lifetimes](fn.main.html#lifetimes)
//! - [Smart Pointers](fn.main.html#smart-pointers)
//! - [Borrow Checker](fn.main.html#borrow-checker)
//! - [Zero-Cost Abstractions](fn.main.html#zero-cost-abstractions)

/// ## 1. Ownership, Borrowing e Lifetimes
///
/// ### Ownership
///
/// - **Cada valor tem um proprietário único (Ownership)**
///
/// Cada valor em Rust tem uma única variável que é seu proprietário.
/// Quando o proprietário sai do escopo, o valor é desalocado automaticamente.
/// Isso significa que não pode haver dois proprietários para o mesmo valor ao mesmo tempo,
/// o que evita duplicações e ambiguidades sobre quem é responsável pela desalocação do valor.
///
/// ```rust
/// fn ownership_example() {
///     let s1 = String::from("hello");
///     let s2 = s1; // s1 é movido para s2
///     // println!("{}", s1); // Erro: s1 não é mais válido
///     println!("{}", s2);
/// }
/// ```
///
/// ### Borrowing
///
/// - **Pode haver um número ilimitado de referências imutáveis, ou apenas uma referência mutável**
///
/// Rust permite emprestimos de valores para outras variáveis ou
/// funções sem transferir a propriedade. Existem dois tipos de referências (empréstimos):
///
/// - **Referências imutáveis (&T):**
///     * Pode haver quantas referências imutáveis forem necessárias,
///     desde que não haja referência mutável ao mesmo tempo.
/// - **Referências mutáveis (&mut T):**
///     * Só pode haver uma referência mutável de cada vez,
///     e nenhuma referência imutável pode coexistir com ela.
///
/// ```rust
/// fn borrowing_example() {
///     let s1 = String::from("hello");
///     let len = calculate_length(&s1); // s1 é emprestado para a função
///     println!("The length of '{}' is {}.", s1, len);
/// }
///
/// fn calculate_length(s: &String) -> usize {
///     s.len()
/// }
/// ```
///
/// ### Lifetimes
///
/// - **As referências sempre devem ser válidas**
///
/// Rust usa lifetimes para garantir que todas as referências sejam válidas enquanto estão em uso.
/// Isso significa que o tempo de vida (lifetime) de uma referência não pode ultrapassar o
/// tempo de vida do valor ao qual ela se refere. Lifetimes são anotados explicitamente
/// pelo compilador ou pelo programador para garantir essa segurança.
///
/// ```rust
/// fn lifetimes_basic_example() {
///     let r;
///     {
///         let x = 5;
///         r = &x; // 'x' não vive o suficiente
///     }
///     // println!("r: {}", r); // Erro de compilação
/// }
/// ```
///
/// - **Cada valor tem um único proprietário:**
///     * Quando o proprietário sai do escopo, o valor é desalocado.
/// - **Pode haver múltiplas referências imutáveis ou uma única referência mutável:**
///     * Não pode haver uma referência mutável e uma referência imutável ao mesmo tempo.
/// - **As referências devem ser sempre válidas:**
///     * O tempo de vida das referências não pode ultrapassar o tempo de vida dos valores aos quais se referem
///

/// ## 3. Smart Pointers
///
/// ### Box
///
/// `Box` permite alocar memória no heap enquanto mantém um ponteiro no stack.
/// É usado para dados grandes ou tipos cujo tamanho não é conhecido em tempo de compilação.
///
/// ```rust
/// fn box_example() {
///     let b = Box::new(5);
///     println!("b = {}", b);
/// }
/// ```
///
/// ### Rc e Arc
///
/// `Rc` (Reference Counted) permite múltiplas referências imutáveis a um dado no heap,
/// enquanto `Arc` (Atomic Reference Counted) permite isso em um contexto multi-thread,
/// com contagem de referências segura para concorrência.
///
/// ```rust
/// use std::rc::Rc;
///
/// fn rc_example() {
///     let a = Rc::new(5);
///     let b = Rc::clone(&a);
///     println!("a = {}, b = {}", a, b);
/// }
/// ```

/// ## 4. Borrow Checker
///
/// ### Mutabilidade
///
/// Rust garante que apenas uma referência mutável ou múltiplas referências imutáveis estejam ativas ao mesmo tempo,
/// prevenindo condições de corrida e outros problemas de concorrência.
///
/// ```rust
/// fn mutability_example() {
///     let mut s = String::from("hello");
///     change(&mut s);
///     println!("{}", s);
/// }
///
/// fn change(some_string: &mut String) {
///     some_string.push_str(", world");
/// }
/// ```
///
/// ### Split Borrows
///
/// Rust não permite referências mutáveis enquanto existirem referências imutáveis,
/// garantindo segurança em tempo de compilação.
///
/// ```rust
/// fn split_borrows_example() {
///     let mut s = String::from("hello");
///     let r1 = &s;
///     let r2 = &s;
///     // let r3 = &mut s; // Erro de compilação
///     println!("{} and {}", r1, r2);
/// }
/// ```

/// ## 5. Zero-Cost Abstractions
///
/// ### Iteradores
///
/// Iteradores em Rust são zero-cost abstractions, ou seja, são otimizados pelo compilador
/// para não introduzir overhead adicional em tempo de execução.
///
/// ```rust
/// fn iterator_example() {
///     let v = vec![1, 2, 3];
///     let v_iter = v.iter();
///     for val in v_iter {
///         println!("{}", val);
///     }
/// }
/// ```
///
/// ### Custom Iterator
///
/// Implementar iteradores customizados em Rust permite criar abstrações poderosas sem comprometer a performance,
/// beneficiando-se das otimizações do compilador.
///
/// ```rust
/// struct Counter {
///     count: u32,
/// }
///
/// impl Counter {
///     fn new() -> Counter {
///         Counter { count: 0 }
///     }
/// }
///
/// impl Iterator for Counter {
///     type Item = u32;
///
///     fn next(&mut self) -> Option<Self::Item> {
///         self.count += 1;
///         if self.count < 6 {
///             Some(self.count)
///         } else {
///             None
///         }
///     }
/// }
///
/// fn custom_iterator_example() {
///     let counter = Counter::new();
///     for val in counter {
///         println!("{}", val);
///     }
/// }
/// ```

fn main() {}