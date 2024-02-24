use lazy_static::lazy_static;

lazy_static! {
    /// 128 common bigrams
    static ref BIGRAMS: &'static str = "intherreheanonesorteattistenntartondalitseediseangoulecomeneriroderaioicliofasetvetasihamaecomceelllcaurlachhidihofonsotacnarssoprrtsassusnoiltsemctgeloeebetrnipeiepancpooldaadviunamutwimoshyoaiewowosfiepttmiopiaweagsuiddoooirspplscaywaigeirylytuulivimabty";
    /// 256 common English words of length four letters or more.
    static ref WORDS: Vec<&'static str> = vec!["that", "this", "with", "from", "your", "have", "more", "will", "home",
"about", "page", "search", "free", "other", "information", "time", "they",
"what", "which", "their", "news", "there", "only", "when", "contact", "here",
"business", "also", "help", "view", "online", "first", "been", "would", "were",
"some", "these", "click", "like", "service", "than", "find", "date", "back",
"people", "list", "name", "just", "over", "year", "into", "email", "health",
"world", "next", "used", "work", "last", "most", "music", "data", "make",
"them", "should", "product", "post", "city", "policy", "number", "such",
"please", "available", "copyright", "support", "message", "after", "best",
"software", "then", "good", "video", "well", "where", "info", "right", "public",
"high", "school", "through", "each", "order", "very", "privacy", "book", "item",
"company", "read", "group", "need", "many", "user", "said", "does", "under",
"general", "research", "university", "january", "mail", "full", "review",
"program", "life", "know", "days", "management", "part", "could", "great",
"united", "real", "international", "center", "ebay", "must", "store", "travel",
"comment", "made", "development", "report", "detail", "line", "term", "before",
"hotel", "send", "type", "because", "local", "those", "using", "result",
"office", "education", "national", "design", "take", "posted", "internet",
"address", "community", "within", "state", "area", "want", "phone", "shipping",
"reserved", "subject", "between", "forum", "family", "long", "based", "code",
"show", "even", "black", "check", "special", "price", "website", "index",
"being", "women", "much", "sign", "file", "link", "open", "today", "technology",
"south", "case", "project", "same", "version", "section", "found", "sport",
"house", "related", "security", "both", "county", "american", "game", "member",
"power", "while", "care", "network", "down", "computer", "system", "three",
"total", "place", "following", "download", "without", "access", "think",
"north", "resource", "current", "media", "control", "water", "history",
"picture", "size", "personal", "since", "including", "guide", "shop",
"directory", "board", "location", "change", "white", "text", "small", "rating",
"rate", "government", "child", "during", "return", "student", "shopping",
"account", "site", "level", "digital", "profile", "previous", "form", "event",
"love", "main", "another", "class", "still"];
}


///  Compress the string 's' of 'len' bytes and stores the compression
//   result in 'dst' for a maximum of 'dstlen' bytes.
pub fn compress(s: &str) -> Option<Vec<u8>> {
    let mut dst: Vec<u8> = vec![];
    let mut verblen = 0u8;
    let s_bytes = s.as_bytes();
    let mut cursor = 0;

    while cursor < s_bytes.len() {
        let mut matched = false;

        if s_bytes.len() - cursor >= 4 {
            for (i, w) in WORDS.iter().enumerate() {
                if s[cursor..].starts_with(w) {
                    let escape_code = match s.as_bytes()[cursor..].get(w.len()) {
                        Some(&b' ') => 7,
                        _ => 6,
                    };

                    dst.push(escape_code);
                    dst.push(i as u8);
                    cursor += w.len();
                    verblen = 0;
                    matched = true;
                    break;
                }
            }
        }

        if matched {
            continue;
        }

        if s_bytes.len() - cursor >= 2 {
            let bigram_slice = &BIGRAMS.as_bytes()[..BIGRAMS.len() - 1];
            for (i, bigram) in bigram_slice.chunks(2).enumerate() {
                if s_bytes[cursor..cursor + 2] == *bigram {
                    dst.push(0x80 | i as u8);
                    cursor += 2;
                    verblen = 0;
                    matched = true;
                    break;
                }
            }
        }

        if matched {
            continue;
        }

        let byte = s_bytes[cursor];
        if !(0x01..=0x08).contains(&byte) {
            dst.push(byte);
            cursor += 1;
            verblen = 0;
        } else {
            verblen += 1;
            if verblen == 1 {
                dst.extend(&[verblen, byte]);
            } else {
                let len_idx = dst.len() - verblen as usize - 1;
                dst[len_idx] = verblen;
                dst.push(byte);
                if verblen == 5 {
                    verblen = 0;
                }
            }
            cursor += 1;
        }
    }

    Some(dst)
}


/// Decompress the string 'c' of 'clen' bytes and stores the decompression
/// result in String 's'.
pub fn decompress(c: &[u8]) -> Option<String> {
    let mut res = String::new();
    let bigrams_bytes = BIGRAMS.as_bytes();
    let mut i = 0;

    while i < c.len() {
        match c[i] {
            0x80..=0xFF => {
                let index = ((c[i] & 0x7F) as usize) * 2;
                if index + 1 < bigrams_bytes.len() {
                    res.push(bigrams_bytes[index] as char);
                    res.push(bigrams_bytes[index + 1] as char);
                    i += 1;
                } else {
                    return None; // Invalid bigram index
                }
            }
            0x01..=0x05 => {
                let length = c[i] as usize;
                if i + length < c.len() {
                    res.extend(c[i + 1..i + 1 + length].iter().map(|&b| b as char));
                    i += length + 1;
                } else {
                    return None; // Invalid byte range for verbatim sequence
                }
            }
            0x06..=0x08 => {
                if i + 1 < c.len() {
                    let index = c[i + 1] as usize;
                    if let Some(word) = WORDS.get(index) {
                        res.push_str(word);
                        if c[i] == 0x07 || c[i] == 0x08 {
                            res.push(' ');
                        }
                        i += 2;
                    } else {
                        return None; // Invalid word index
                    }
                } else {
                    return None; // Not enough bytes for a word escape code
                }
            }
            _ => {
                res.push(c[i] as char);
                i += 1;
            }
        }
    }

    Some(res)
}
