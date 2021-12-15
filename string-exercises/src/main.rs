mod reverse;
mod anagram;

fn main() {
    let reversed = reverse::reverse1("teste");
    println!("{}", reversed);

    let anagrams = anagram::anagrams_for("hello", &["ffd", "fdsgfdg", "olleh", "lloHe"]);
    println!("{:?}", anagrams);
}
