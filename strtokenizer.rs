/*
* strtokenizer.rs - Copyright (c) 2013 Letang Jeremy (letang.jeremy@gmail.com).
*
* This software is provided 'as-is', without any express or implied warranty.
* In no event will the authors be held liable for any damages arising from
* the use of this software.
*
* Permission is granted to anyone to use this software for any purpose,
* including commercial applications, and to alter it and redistribute it
* freely, subject to the following restrictions:
*
* 1. The origin of this software must not be misrepresented; you must not claim
*    that you wrote the original software. If you use this software in a product,
*    an acknowledgment in the product documentation would be appreciated but is
*    not required.
*
* 2. Altered source versions must be plainly marked as such, and must not be
*    misrepresented as being the original software.
* 
* 3. This notice may not be removed or altered from any source distribution.
*/


/*!
* StringTokenizer
*
* # Basic Example :
* ~~~
*
* let mut st = strtokenizer::StringTokenizer::new_with_str("A Simple string to tokenize!");
* while st.has_token() {
*     match st.token {
*         strtokenizer::SpecialChar   => io::println(fmt!("SPECIAL CHAR : %c", st.specialChar)),
*         strtokenizer::Word          => io::println(fmt!("WORD : %s", st.word)),
*         strtokenizer::Number        => io::println(fmt!("NUMBER : %s", st.number)),
*         strtokenizer::KeyWord       => io::println(fmt!("KEYWORD : %s", st.keyword)),
*         strtokenizer::NoToken       => {}
*        }
*    }
*
* ~~~
*/

#[link(name = "strtokenizer",
       vers = "0.0.1",
       author = "letang.jeremy@gmail.com",
       uuid = "5CB40369-7C8F-4D70-A419-1FEA19AAF46D")];
#[crate_type = "lib"];

#[cfg(test)]
use std::io;
#[cfg(test)]
use std::path::PosixPath;

use std::str;

/**
* The CommentsTypes enum, define the differents comments types availables.
*/
pub enum CommentsTypes {
    CplusplusComments,
    CComments,
    AllComments,
    NoComments
}

/**
* The Token enum, define the differents token availables from StringTokenizer.
*/
pub enum Token {
    Word,
    KeyWord,
    Number,
    SpecialChar,
    NoToken
}

/**
* The StringTokenizer struct.
*
* # Public attributes
* * token       - Token enum, define the type of the current token
* * number      - Contain the current token if the type of the token is Number
* * word        - Contain the current token if the type of the token is Word
* * specialChar - Contain the current token if the type of the token is specialChar
*/
pub struct StringTokenizer {
    priv datas : ~[char],
    priv pos : uint,
    priv keyWords : ~[~str], 
    priv delimiters : ~[char],
    priv specialChars : ~[char],
    priv comments : CommentsTypes,
    priv returnIsToken : bool,
    priv ignoreEscapeChar : bool,
    priv multiCommentBegin : ~str,
    priv multiCommentEnd : ~str,
    priv singleComment : ~str,
    token : Token,
    number : ~str,
    word : ~str,
    keyword : ~str,
    specialChar : char
}

/**
* Convert an owned str to an owned vector of chars.
*
* Return the owned vector of chars.
*/
pub fn str_to_vec(string : ~str) -> ~[char]{
    let mut bytes = ~[];
    let mut i = 0;

    while i < string.len() {
        bytes.push(string.char_at(i));
        i+= 1;
    }
    bytes
}

/**
* Methods for struct StringTokenizer
*/
impl StringTokenizer {
    /**
    * Create a new StringTokenizer object.
    *
    * # Arguments
    * * datas - The vector of chars to tokenize
    *
    * Return a new instance of StringTokenizer.
    */
    pub fn new(datas : ~[char]) -> StringTokenizer {
        let mut st = StringTokenizer {
            datas : datas,
            pos : 0,
            keyWords : ~[],
            delimiters : ~[],
            specialChars : ~[],
            comments : NoComments,
            returnIsToken : true,
            ignoreEscapeChar : false,
            multiCommentBegin : ~"",
            multiCommentEnd : ~"",
            singleComment : ~"",
            token : NoToken,
            number : ~"",
            word : ~"",
            keyword : ~"",
            specialChar : 0 as char
        };
        st.initialize();
        st
    }

    /**
    * Create a new StringTokenizer object.
    *
    * # Arguments
    * * datas - The string to tokenize
    *
    * Return a new instance of StringTokenizer.
    */
    pub fn new_with_str(datas : ~str) -> StringTokenizer {
        let mut st = StringTokenizer {
            datas : str_to_vec(datas),
            pos : 0,
            keyWords : ~[],
            delimiters : ~[],
            specialChars : ~[],
            comments : NoComments,
            returnIsToken : true,
            ignoreEscapeChar : false,
            multiCommentBegin : ~"",
            multiCommentEnd : ~"",
            singleComment : ~"",
            token : NoToken,
            number : ~"",
            word : ~"",
            keyword : ~"",
            specialChar : 0 as char
        };
        st.initialize();
        st
    }
    
    /**
    * Private function, initalize the StringTokenizer with a basic dictionnary
    */
    fn initialize(&mut self) -> () {
        self.delimiters.push('\t');
        self.delimiters.push(' ');
        self.specialChars.push('{');
        self.specialChars.push('}');
        self.specialChars.push('[');
        self.specialChars.push(']');
        self.specialChars.push('(');
        self.specialChars.push(')');
        self.specialChars.push('=');
        self.specialChars.push('!');
        self.specialChars.push('<');
        self.specialChars.push('>');
        self.specialChars.push('&');
        self.specialChars.push('^');
        self.specialChars.push('|');
        self.specialChars.push('+');
        self.specialChars.push('-');
        self.specialChars.push('/');
        self.specialChars.push('%');
        self.specialChars.push('*');
        self.specialChars.push(';');
        self.specialChars.push('?');
        self.specialChars.push(':');
        self.specialChars.push(',');
        self.consume_delimiters();
    }
    
    /**
    * Private function consume char while there is delimiters.
    */
    fn consume_delimiters(&mut self) -> () {
        while self.pos < self.datas.len() 
            && self.is_delimiter(self.datas[self.pos]) { 
            self.pos += 1;
        }
    }

    /**
    * Private function test if a char is a delimiter or not
    *
    * # Arguments
    * * testChar - The character to test
    *
    * Return true if the char is a delimiter, false otherwise
    */
    fn is_delimiter(&mut self, testChar : char) -> bool {
        for self.delimiters.iter().advance |delim| {
            if *delim == testChar {
                return true;
            }
        }
        return false
    }

    /**
    * Private function, test if a char is a special char.
    *
    * # Arguments
    * * testChar - The character to test
    *
    * Return true if the char is a special char, false otherwise
    */
    fn is_special_char(&mut self, testChar : char) -> bool {
        for self.specialChars.iter().advance |spec| {
            if *spec == testChar {
                return true;
            }
        }
        return false
    }

    /**
    * Update the vector of data to tokenize by a new one.
    */
    pub fn set_datas(&mut self, datas : ~[char]) -> () {
        self.datas = datas;
        self.pos = 0;
    }

    /**
    * Update the vector of data to tokeny by a new string.
    */
    pub fn set_datas_with_str(&mut self, datas : ~str) -> () {
        self.datas = str_to_vec(datas)
    }
    
    /**
    * Add a new key word to the StringTokenizer dictionnary.
    *
    * # Arguments
    * * keyword - A string who contains the new keyword to add
    */
    pub fn add_keyword(&mut self, keyword : ~str) -> () {
        self.keyWords.push(keyword)
    }

    /**
    * Add a new delimiter to the StringTokenizer dictionnary.
    *
    * # Default
    * * '\t' 
    * * ' '
    * 
    * # Arguments
    * * delimiter - The char containing the delimiter to add
    */
    pub fn add_delimiter(&mut self, delimiter : char) -> () {
        self.delimiters.push(delimiter)
    }
    
    /**
    * Add a new special char to the StringTokenizer dictionnary.
    * 
    * # Default
    * * '{' '}' '(' ')' '[' ']'
    * * '=' '!' '<' '>' '&' '^' '|'
    * * '+' '-' '/' '%' '*'
    * * ';' '?' ':' ','
    * 
    * # Arguments
    * * specialchar - The new char to add to the special chars list
    */
    pub fn add_specialchar(&mut self, specialchar : char) -> () {
        self.specialChars.push(specialchar)
    }

    /**
    * Set the type of comments handled by the StringTokenizer.
    *
    * # Default
    * * No comments are handled by default 
    *
    * # Arguments
    * * comments - The new type of comments to handle
    */
    pub fn set_comments(&mut self, comments : CommentsTypes) -> () {
        self.comments = comments
    }

    /**
    * Define if the '\n' char is a new token or a delimiter
    *
    * # Default
    * * By default '\n' is a token
    * 
    * # Arguments
    * isToken - true if it's a token false otherwise
    */
    pub fn set_new_line_as_token(&mut self, isToken : bool) -> () {
        self.returnIsToken = isToken;
        if isToken == true {
            self.add_specialchar('\n');
        }
        else {
            self.add_delimiter('\n');
        }
            
    }
    
    /**
    * Ignore or not the escape char.
    *
    * # Default
    * * By default escape char is not ignored
    *
    * # Example
    * * ignored - This string |"This is a message :\"Hello World\""| provide these tokens |"This is a message : \" - Hello - World\ - ""|
    * * not ignored - This string |"This is a message :\"Hello World\""| provide this tokens |"This is a message :\"Hello World\""|
    * 
    * # Arguments
    * * ignore - Ture if escape char must be ignored, false otherwise
    */
    pub fn ignore_escape_char(&mut self, ignore : bool) -> () {
        self.ignoreEscapeChar = ignore
    }

    /**
    * Reset all the settings contained on the StringTokenizer
    * ( datas / keywords / delimiters / specialchars / tokens ).
    */
    pub fn reset_settings(&mut self) -> () {
        self.delimiters.clear();
        self.keyWords.clear();
        self.datas.clear();
        self.pos = 0;
        self.specialChars.clear();
        self.comments = NoComments;
        self.returnIsToken = true;
        self.token = NoToken;
        self.number = ~"";
        self.word = ~"";
        self.keyword = ~"";
        self.multiCommentBegin = ~"";
        self.multiCommentEnd = ~"";
        self.singleComment = ~"";
        self.specialChar = 0 as char;
    }

    /**
    * Set a custom multi line comment
    *
    * # Arguments
    * * commentBegin - The string who represent the begin of the comment.
    * * commentEnd - The string who represent the end of the comment.
    */
    pub fn set_multi_line_custom_comment(&mut self, commentBegin : ~str, commentEnd : ~str) -> () {
        self.multiCommentBegin = commentBegin;
        self.multiCommentEnd = commentEnd;
    }
    
    /**
    * Set a custom single line comment.
    *
    * # Arguments
    * * commentBegin - The str who represent the begin of a custom comment
    */
    pub fn set_single_line_custom_comment(&mut self, commentBegin : ~str) -> () {
        self.singleComment = commentBegin;
    }
    
    /**
    * Private function, retrieve a string in double cote.
    *
    * Return the string contained between two simple cotes.
    */
    fn get_double_cote_string(&mut self) -> ~str {
        let mut tstr : ~[char] = ~[];
        let mut end = false;
        let mut cntS = 0;
        let mut nxtD = false;

        tstr.push(self.datas[self.pos]);
        self.pos += 1;
        while self.pos < self.datas.len() 
            && !end {
            if nxtD == true {
                nxtD = false;
                cntS = 0;
            }
            if self.datas[self.pos] == '\\' {
                cntS += 1;
                if cntS == 3 {
                    cntS = 1;
                }
            }
            else if cntS > 0{
                nxtD = true;
            }
            tstr.push(self.datas[self.pos]);
            if self.datas[self.pos] == '"'
                && (cntS == 2 || cntS == 0) {
                end = true;
            }
            self.pos += 1;
        } 
        str::from_chars(tstr)
    }

    /**
    * Get a String in simple cote.
    *
    * Return all the string contained between two simple cotes.
    */
    fn get_simple_cote_string(&mut self) -> ~str {
        let mut tstr : ~[char] = ~[];
        let mut end = false;
        let mut cntS = 0;
        let mut nxtD = false;
        
        tstr.push(self.datas[self.pos]);
        self.pos += 1;
        while self.pos < self.datas.len() 
            && !end {
            if nxtD == true {
                nxtD = false;
                cntS = 0;
            }
            if self.datas[self.pos] == '\\' {
                cntS += 1;
                if cntS == 3 {
                    cntS = 1;
                }
            }
            else if cntS > 0{
                nxtD = true;
            }
            tstr.push(self.datas[self.pos]);
            if self.datas[self.pos] == '\''
                && (cntS == 2 || cntS == 0) {
                end = true;
            }
            self.pos += 1;
        } 
        str::from_chars(tstr)
    }

    /**
    * Private function, get the next from the current position.
    *
    * Return the word read.
    */
    fn get_word(&mut self) -> ~str {
        let mut tstr : ~[char] = ~[];
        
        if self.datas[self.pos] == '"' {
            return self.get_double_cote_string();
        }
        else if self.datas[self.pos] == '\'' {
            return self.get_simple_cote_string();
        } 
        while self.pos < self.datas.len()
            && !self.is_delimiter(self.datas[self.pos])
            && !self.is_special_char(self.datas[self.pos]){
            tstr.push(self.datas[self.pos]);
            self.pos += 1;
        }
        str::from_chars(tstr)
    }

    /**
    * Private function, handle C++ style comment
    *
    * Return true if a comment is found, false otherwise
    */
    fn c_plus_plus_comments(&mut self) -> bool {
        let mut end = false;
        if self.pos <= self.datas.len() - 2 {
            if self.datas[self.pos] == '/'
                && self.datas[self.pos + 1] == '/' {
                while self.pos < self.datas.len() 
                    && !end {
                    if self.datas[self.pos] == '\n' {
                        end = true
                    }
                    self.pos += 1;
                }
                return true;
            }
                    else {
                        return false;
                    }
                    
                }
        else {
            return false;
        }
    }
    
    /**
    * Private function, handle C style comment erasing
    *
    * Return true if a C style comment is found, false otherwise
    */
    fn c_comments(&mut self) -> bool {
        let mut end = false;
        if self.pos <= self.datas.len() - 2 {
            if self.datas[self.pos] == '/'
                && self.datas[self.pos + 1] == '*' {
                self.pos += 2;
                while self.pos < self.datas.len()
                    && !end {
                    if self.pos + 1 <= self.datas.len()
                        && (self.datas[self.pos] == '*' && self.datas[self.pos + 1] == '/') {
                        end = true;
                        if (self.pos < self.datas.len()) {
                            self.pos += 1;
                        }
                    }
                    self.pos += 1;
                }
                return true;
            }
            
            else {
                return false;
            }
        }
        else {
            return false;
        }
        
    }

    /**
    * Private function, Check if there is comments and delete them
    *
    * Return true if a comment is found.
    */
    fn has_comments(&mut self) -> bool {
        if self.pos == self.datas.len() {
            return false;
        }
        else {
            if self.c_comments() {
                return true;
            }
            else if self.c_plus_plus_comments() {
                return true;
            }
            else {
                return false;
            }
        }
    }
    
    /**
    * Private function, consume all delimiters or comments
    */
    fn clean_for_next_token(&mut self) -> bool {
        self.consume_delimiters();
        while self.has_comments() {
            self.consume_delimiters();
        }
        if self.pos == self.datas.len() {
            false
        }
        else {
            true
        }
    }

    /**
    * Private function, check if the found word is a number
    *
    * Return true if it's a number, false otherwise
    */
    fn is_number(&mut self) -> bool {
        let mut dot = 0;
 
        for self.word.iter().advance |schar| {
            if schar == '.' {
                dot += 1;
            }
            if !schar.is_digit() {
                return false;
            }
        }
        if dot > 1 {
            return false;
        }
        return true;
    }


    /**
    * Private functions, check if the found word is a keyword
    *
    * Return true if it is a keyword, false otherwise
    */
    fn is_keyword(&self) -> bool {
        for self.keyWords.iter().advance |keyword| {
            if keyword == &self.word {
                return true;
            }
        }
        return false
    }

    /**
    * Try to find new tokens.
    *
    * Return true if there is token, false otherwise.
    */
    pub fn has_token(&mut self) -> bool {
        if self.pos == self.datas.len() {
            return false;
        }
        else {
            if !self.clean_for_next_token() {
                return false;
            } 
            if self.is_special_char(self.datas[self.pos]) {
                self.specialChar = self.datas[self.pos];
                self.token = SpecialChar;
                self.pos += 1;
                return true;
            }
            else {
                let tstr = self.get_word();
                self.word = tstr;
                if self.is_number() {
                    self.token = Number;
                    self.number = self.word.clone();
               }
                else if self.is_keyword() {
                    self.token = KeyWord;
                    self.keyword = self.word.clone();
                }
                else {
                    self.token = Word;
                }
                return true;
            }
        }
    }
}

/**
* Implementation of trait Clone, clone a StringTokenizer instance
*/
impl Clone for StringTokenizer {
    fn clone(&self) -> StringTokenizer {
        StringTokenizer {
            datas : self.datas.clone(),
            pos : self.pos,
            keyWords : self.keyWords.clone(),
            delimiters : self.delimiters.clone(),
            specialChars : self.specialChars.clone(),
            comments : self.comments,
            returnIsToken : true,
            ignoreEscapeChar : false,
            multiCommentBegin : self.singleComment.clone(),
            multiCommentEnd : self.singleComment.clone(),
            singleComment : self.singleComment.clone(),
            token : NoToken,
            number : self.number.clone(),
            word : self.word.clone(),
            keyword : self.keyword.clone(),
            specialChar : self.specialChar
        }
    }
}

#[test]
fn test_with_strotkenierrs() -> () {
    let path = ~PosixPath("./strtokenizer.rs");
    let file = match io::read_whole_file_str(path) {
        Ok(file)        => file,
        Err(error)      => fail!(fmt!("Error during file reading :\n%s", error))
    };
    let i = 42;
    let j = 42.42;
    let h = 2345678;
    //io::println(fmt!("File main.rs :\n\n%s\nTokens :\n\n", file));
    let mut st = StringTokenizer::new_with_str(file);
    st.set_comments(AllComments);
    st.set_new_line_as_token(false);
    st.add_keyword(~"let");
    while st.has_token() {
        match st.token {
            SpecialChar   => io::println(fmt!("SPECIAL CHAR : %c", st.specialChar)),
            Word          => io::println(fmt!("WORD : %s", st.word)),
            Number        => io::println(fmt!("NUMBER : %s", st.number)),
            KeyWord       => io::println(fmt!("KEYWORD : %s", st.keyword)),
            NoToken       => {}
        }
    }
}
