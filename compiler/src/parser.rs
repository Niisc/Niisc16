use crate::lexer::*;
use std::iter::Enumerate;

pub struct Parser<'a> {
    tokens: &'a mut Enumerate<std::slice::Iter<'a, Token>>,
    current_token: Option<&'a Token>,
    peek_token: Option<&'a Token>,
    label_is_declared: Vec<&'a str>,
    label_is_gotoed: Vec<&'a str>,
    symbols: Vec<&'a str>,

}

impl<'a> Parser <'a> {
    pub fn init(tok: &'a mut Enumerate<std::slice::Iter<'a, Token>>) -> Parser<'a> {
        Parser {
            tokens : tok,
            current_token : None,
            peek_token: None, 
            label_is_declared: Vec::new(),
            label_is_gotoed: Vec::new(),
            symbols: Vec::new(),
        }

    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token;
        self.peek_token = self.tokens.next().unzip().1;
    }

    pub fn program(&mut self) {

        self.next_token();
        self.next_token();

        while self.current_token.unwrap().token == TokenType::NEWLINE {
            self.next_token();
        }

        while self.current_token.unwrap().token != TokenType::EOF {
            self.statement();
        }

        for label in self.label_is_gotoed.iter() {
            if !self.label_is_declared.contains(label){
                panic!("attempting to GOTO to undetermined label: {}", label);
            }
        }
    }
    
    fn statement(&mut self) {
       
        match self.current_token.unwrap().token {
            
            TokenType::PRINT => {
                self.next_token();
                
                if self.current_token.unwrap().token == TokenType::STRING {
                    self.next_token();

                }else {
                    self.expression();
                }
            }

            TokenType::IF => {
                self.next_token();
                self.comparison();
                
                self.match_token(TokenType::THEN);
                self.new_line();

                while self.current_token.unwrap().token != TokenType::ENDIF {
                    self.statement()
                }

                self.match_token(TokenType::ENDIF);

            }

            TokenType::WHILE => {
                self.next_token();
                self.comparison();

                self.match_token(TokenType::REPEAT);
                self.new_line();

                while self.current_token.unwrap().token != TokenType::ENDWHILE {
                    self.statement();
                }

                self.match_token(TokenType::ENDWHILE);
            }

            TokenType::LABEL => {
                self.next_token();

                if self.label_is_declared.contains(&self.current_token.unwrap().data.as_str()) {
                    panic!("label already declared");
                }
                self.label_is_declared.push(&self.current_token.unwrap().data);
                
                self.match_token(TokenType::IDENT);
            }

            TokenType::GOTO => {
                self.next_token();
                self.label_is_gotoed.push(&self.current_token.unwrap().data.as_str());
                self.match_token(TokenType::IDENT);
            }

            TokenType::LET => {
                self.next_token();
                if !self.symbols.contains(&self.current_token.unwrap().data.as_str()) {
                    self.symbols.push(&self.current_token.unwrap().data);
                }

                self.match_token(TokenType::IDENT);
                self.match_token(TokenType::EQ);

                self.expression();
            }

            TokenType::INPUT => {
                self.next_token();

                if !self.symbols.contains(&self.current_token.unwrap().data.as_str()) {
                    self.symbols.push(&self.current_token.unwrap().data);
                }
                self.match_token(TokenType::IDENT);
            }


            _=> panic!("problem in match parser")
        }
        self.new_line();
    }
    
    fn comparison(&mut self) {
        self.expression();

        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            panic!("expected comparison operator")
        }

        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }

    }

    fn expression(&mut self) {
        self.term();

        while self.current_token.unwrap().token == TokenType::PLUS ||  self.current_token.unwrap().token == TokenType::MINUS {
            self.next_token();
            self.unary();
        }
    }

    fn term(&mut self) {
        self.unary();

        while self.current_token.unwrap().token == TokenType::ASTERISK || self.current_token.unwrap().token == TokenType::SLASH {
            self.next_token();
            self.unary();
        } 

    }

    fn unary(&mut self) {
        if self.current_token.unwrap().token == TokenType::PLUS || self.current_token.unwrap().token == TokenType::MINUS {
            self.next_token()
        } 
        self.primary();
    }

    fn primary(&mut self) {
        if self.current_token.unwrap().token == TokenType::NUMBER {
            self.next_token()
        } else if self.current_token.unwrap().token == TokenType::IDENT {
            if !self.symbols.contains( &self.current_token.unwrap().data.as_str()){
                panic!("refferencing variable before assignment: {}",&self.current_token.unwrap().data.as_str() )
            }
            self.next_token();
        }else {
            panic!("unexpected error at: {}",&self.current_token.unwrap().data.as_str());
        }
    }
    

    fn new_line(&mut self) {
        self.match_token(TokenType::NEWLINE);
        while self.current_token.unwrap().token == TokenType::NEWLINE {
            self.next_token();
        }
    }

    fn match_token(&mut self, token: TokenType) {
        if self.current_token.unwrap().token != token {
            panic!("expected {} found {}", token, self.current_token.unwrap().token );
        }
        self.next_token();
    }

    fn is_comparison_operator(&mut self) -> bool {
        self.current_token.unwrap().token == TokenType::GT || self.current_token.unwrap().token == TokenType::GTEQ || self.current_token.unwrap().token == TokenType::LT || self.current_token.unwrap().token == TokenType::LTEQ || self.current_token.unwrap().token == TokenType::EQEQ || self.current_token.unwrap().token == TokenType::NOTEQ
    }
    
}

