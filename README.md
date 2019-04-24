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

`let tulisan = candidate.to_lowercase(); `

`candidate` merupakan parameter dari fungsi `check` yang merupakan kata input 

Kedua kita jadikan kata input tadi menjadi sebuah vector, yang berarti di dalam setiap kolom vektornya terdapat satu karakter dari input kata nya. Misal `"halo"` menjadi `['h', 'a', 'l', 'o']`. kodenya sebagai berikut:

`let char_vec: Vec<char> = tulisan.chars().collect(); `

ketiga kita perlu memfilter apakah sebuah huruf dalam satu kata input ada yang memiliki kesamaan. Jika YA maka jawabannya adalah `False`, karena itu bukan kata Isogram. Jika TIDAK maka sebaliknya. Oleh karena itu, kita perlu memeriksa satu persatu karakter yang ada didalam vector `char_vec`. Caranya dengan melakukan pengulangan sesuai dengan banyaknya jumlah huruf pada kata inputan.

`for i in 0..char_vec.len();`

`0..char_vec.len();` memiliki arti pengulangan akan berulang sebanyak 0 hingga panjangnya vector.

dalam pengulangan ini, kita mencari karakter pertama dalam vektor yang akan kita jadikan patokan pencarian karakter lainnya. Misalnya `['h', 'a', 'l', 'o']`. kita ambil karakter pertama untuk dijadikan patokan yaitu huruf `'h'` dengan variable baru bernama tes `let tes = char_vec[i];`. kemudian kita cocokan dengan karakter lainnya didalam vektor.  karena dalam pengulangan ini kita hanya bisa mematokkan satu karakter saja, maka perlunya membuat pengulangan dalam pengulangan.  

`for j in 0..char_vec.len()-1`

agar pengulangan terus berlanjut, maka perlunya menyimpan letak yang sudah dicari sebelumnya dengan kode `let titik = i;`. jadi `titik` merupakan `0` jika sebelumnya diambil huruf `'h'` (karena vector 0 merupakan huruf h). maka selanjutnya berlanjut ke titik `1` dan seterusnya

```if char_vec[j] == tes{
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
```

Selanjutnya kita bisa memfilter karakter satu persatu dengan `if else`. dengan cara mencari tahu apakah karakter pertama sama dengan karakter `tes` (dari pengulangan yang sebelumnya). Jika sama, maka dites kembali karena ada dua kemungkinan. kemungkinan pertama yaitu pengujian dengan diri sendiri dan yang kedua pengujian dengan karakter lain. Jika pengujian dengan diri sendiri terjadi, maka seharusnya di-skip karena bisa terhitung double character. maka returnnya `count = count` alias tidak berpengaruh. Sebaliknya, jika pengujian tidak dengan diri sendiri, maka baru akan dihitung jika karakter ada yang double.

Tetapi kita perlu memfilter lagi, karakter `' '` spasi dan `'-'` tanda strip bisa dihitung double character jika memang terdapat dalam satu inputan. Padahal yang dihitung sebenarnya hanya karakter huruf saja. maka perlunya kita memfilter dengan cara `if char_vec[j] == ' ' || char_vec[j] == '-'`. 

Setelah pengecekan karakter lain selain huruf, maka barulah kita mendapatkan hasil yang akurat mengenai pencarian persamaan karakter.

`count` berguna untuk menghitung jumlah yang terdapat pada huruf yang sama atau tidak. Maka kita deklarasikan diawal looping. Jika terdapat hasil yang sama setelah filterisasi, maka `count` akan bertambah satu. Jika hasilnya tidak, maka count akan sama hasilnya.

karena count selalu ditambah jika terdapat kesamaan karakter, maka perlunya membuat `if else` untuk me-return apakah kata ini merupakan kata isogram. atau bukan. `true` untuk hasil bahwa kata itu Isogram dan `false` sebaliknya. berikut kodenya:.

```if count > 0 {
    	bolean = false
    }else{
    	bolean = true;
    	count = 0;
    }
    return bolean;
    }```

### Terimakasih :)







