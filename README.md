# Exercism-RustTrack
Perkenalkan saya, Faqihuddin Al Farisi selaku mahasiswa Ilmu Komputer Universitas Negeri Jakarta. Dalam Exercise kali ini, saya berhasil mengerjakan beberapa problem dengan bahasa pemrograman Rust yang dapat di akses pada folder-folder diatas. Problemnya beragam, yang saya kerjakan beberapa bagian Easy dan Medium.

Saya ingin membahas salah satu problem yang telah solved. yaitu problem dengan judul Isogram.

## Isogram 
berikut instruksinya
> _Determine if a word or phrase is an isogram.
> An isogram (also known as a "nonpattern word") is a word or phrase without a repeating letter, however spaces and hyphens are allowed to appear multiple times.
> Examples of isograms:
    lumberjacks,
    background,
    downstream,
    six-year-old.
> The word isograms, however, is not an isogram, because the s repeats._ 

Dari masalah diatas, kita disuruh untuk membuat program dimana program tersebut dapat mengetahui bahwa sebuah kata yang di input merupakan Isogram atau tidak. Contohnya pada kata _"lumberjacks"_. kata Lumberjack sendiri tidak memiliki satupun karakter huruf yang terulang. begitupun kata _Background, Downstream dan Six-year-old_. Berikut kode yang dapat saya solve


   ```rs 
    pub fn check(candidate: &str) -> bool {
    let tulisan = candidate.to_lowercase();
    let char_vec: Vec<char> = tulisan.chars().collect();
    let mut count = 0;

    for i in 0..char_vec.len(){
        let tes = char_vec[i];
        let titik = i;
    	for j in 0..char_vec.len()-1{
    		if char_vec[j] == tes{
    			if titik == j {
    				count = count;
    			}else{
    				if char_vec[j] == ' ' || char_vec[j] == '-'{
    					count = count;
    				}else {
    					count += 1;
    				}
    			}
    		}
    		else{
    			count = count;
    		}
    	}
    }
    
    let mut bolean = true;
    if count > 0 {
    	bolean = false
    }else{
    	bolean = true;
    	count = 0;
    }
    return bolean;
    }

```


Dalam kode diatas, algroritma yang dibangun sebagai berikut. 

Pertama kata input kita jadikan lowercase semua agar tidak terjadi miss (karena rust termasuk sensitive case) dengan menggunakan `to_lowercase();`. 
<p> `let tulisan = candidate.to_lowercase();`
<p> `candidate` </p> merupakan parameter dari fungsi `check`

Kedua kita jadikan kata input tadi menjadi sebuah vector, yang berarti di dalam setiap kolom vektornya terdapat satu karakter dari input kata nya. Misal `"halo"` menjadi `['h', 'a', 'l', 'o']`. 

ketiga

