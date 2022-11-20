#[derive(Clone, Debug)]
pub enum Op{

    Id(bool),
    Negate,
    Conjunktion,
    Disjunktion,
    Implikation,
    Biconditional,

}

type Child = Option<Box<Expression>>;
#[derive(Clone, Debug)]
pub struct Expression{

    left_tree: Child,
    right_tree: Child,
    junktor: Op,

}

impl Expression{

    pub fn new(junktor: Op, left_tree: Expression, right_tree: Expression) -> Self{
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: Some(Box::new(right_tree)), junktor: junktor }
    }

    pub fn NegateNode(left_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: None, junktor: Op::Negate}
    }
    
    pub fn DisjunktionNode(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: Some(Box::new(right_tree)), junktor: Op::Disjunktion}
    }
    
    pub fn ConjunktionNode(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: Some(Box::new(right_tree)), junktor: Op::Conjunktion}
    }

    pub fn ImplikationNode(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: Some(Box::new(right_tree)), junktor: Op::Implikation}
    }
    
    pub fn Implikation(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(Self::NegateNode(left_tree))), right_tree: Some(Box::new(right_tree)), junktor: Op::Disjunktion}
    }

    pub fn BiconditionalNode(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(left_tree)), right_tree: Some(Box::new(right_tree)), junktor: Op::Biconditional}
    }

    pub fn Biconditional(left_tree: Expression, right_tree: Expression) -> Self {
        Expression { left_tree: Some(Box::new(Self::Implikation(left_tree.clone(),right_tree.clone()))), right_tree: Some(Box::new(Self::Implikation(right_tree,left_tree))), junktor: Op::Conjunktion}
    }

    pub fn IdentityNode(value: bool) -> Self{
        Expression { left_tree: None, right_tree: None, junktor: Op::Id(value)}
    }
    
    pub fn collapse(&self) -> bool {
        let mut right_accu: Option<bool> = None;
        let mut left_accu: Option<bool> = None;
        
        if let Some(left) = &self.left_tree {
            left_accu = Some(self.left_tree.as_ref().unwrap().collapse());
        }
        
        if let Some(right) = &self.right_tree {
            right_accu = Some(self.right_tree.as_ref().unwrap().collapse());
        }
        
        let r = if let Some(x) = right_accu { x } else { true };
        let l = if let Some(x) = left_accu { x } else { true };  
        
        match self.junktor {
            Op::Conjunktion => { l & r  }
            Op::Disjunktion => { l || r }
            Op::Negate => { !l }
            Op::Implikation => {(!l) || r}
            Op::Biconditional => {((!l) || r) & ((!r) || l)}
            Op::Id(x) => x 
        }
    }

}
pub struct Formula{

    head: Option<Expression>,

}

impl Formula{

    pub fn new(head: Expression) -> Self {
        Formula { head: Some(head) } 
    }

    pub fn collapse(&self) -> bool{

        self.head.as_ref().unwrap().collapse()

    }

}

#[derive(Debug, Clone)]
pub enum LexItem {
    Paren(char),
    Op(char),
    Variable(char),
    Bool(bool),
}

fn lex(input: &String) -> Result<Vec<LexItem>, String> {

    let mut result = Vec::new();

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0' | '1' => {
                it.next();
                match c {
                    '0' => result.push(LexItem::Bool(false)),
                    '1' => result.push(LexItem::Bool(true)),
                    _ => panic!("bool panic at lexing"),
                }
            }
             '/' | '&' | '|' | '>' | '-' => {
                result.push(LexItem::Op(c));
                it.next();
            }
            '(' | ')' | '[' | ']' | '{' | '}' => {
                result.push(LexItem::Paren(c));
                it.next();
            }
            'a'..='z' => {
                it.next();
                result.push(LexItem::Variable(c));
            }
            ' ' => {
                it.next();
            }
            _ => {
                return Err(format!("unexpected character {}", c));
            }
        }
    }
    Ok(result)
}

pub fn parse(input: &String) -> Result<ParseNode, String> {
    let tokens = lex(input)?;
    parse_expression(&tokens, 0).and_then(|(n, i)| if i == tokens.len() {
        Ok(n)
    } else {
        Err(format!("Expected end of input, found {:?} at {}", tokens[i], i))
    })
}

fn parse_expression(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    //"(x & y) > z"
    let (node_summand, next_pos) = parse_variable(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('+')) => {
            // recurse on the expr
            let mut sum = ParseNode::new();
            sum.entry = GrammarItem::Sum;
            sum.children.push(node_summand);
            let (rhs, i) = parse_expression(tokens, next_pos + 1)?;
            sum.children.push(rhs);
            Ok((sum, i))
        }
        _ => {
            // we have just the summand production, nothing more.
            Ok((node_summand, next_pos))
        }
    }
}

fn parse_junktor(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let (node_term, next_pos) = parse_variable(tokens, pos)?;
    let c = tokens.get(next_pos);
    match c {
        Some(&LexItem::Op('*')) => {
            // recurse on the summand
            let mut product = ParseNode::new();
            product.entry = GrammarItem::Product;
            product.children.push(node_term);
            let (rhs, i) = parse_junktor(tokens, next_pos + 1)?;
            product.children.push(rhs);
            Ok((product, i))
        }
        _ => {
            // we have just the term production, nothing more.
            Ok((node_term, next_pos))
        }
    }
}

fn parse_variable(tokens: &Vec<LexItem>, pos: usize) -> Result<(ParseNode, usize), String> {
    let c: &LexItem = tokens.get(pos)
        .ok_or(String::from("Unexpected end of input, expected paren or number"))?;
    match c {
        &LexItem::Variable() => {
            let mut node = ParseNode::new();
            node.entry = GrammarItem::Variable;
            Ok((node, pos + 1))
        }
        &LexItem::Paren(c) => {
            match c {
                '(' | '[' | '{' => {
                    parse_expression(tokens, pos + 1).and_then(|(node, next_pos)| {
                        if let Some(&LexItem::Paren(c2)) = tokens.get(next_pos) {
                            if c2 == matching(c) {
                                // okay!
                                let mut paren = ParseNode::new();
                                paren.children.push(node);
                                Ok((paren, next_pos + 1))
                            } else {
                                Err(format!("Expected {} but found {} at {}",
                                            matching(c),
                                            c2,
                                            next_pos))
                            }
                        } else {
                            Err(format!("Expected closing paren at {} but found {:?}",
                                        next_pos,
                                        tokens.get(next_pos)))
                        }
                    })
                }
                _ => Err(format!("Expected paren at {} but found {:?}", pos, c)),
            }
        }
        _ => {
            Err(format!("Unexpected token {:?}, expected paren or number", {
                c
            }))
        }
    }
}

#[derive(Debug, Clone)]
pub enum GrammarItem{

    Id(bool),
    Negate,
    Conjunktion,
    Disjunktion,
    Implikation,
    Biconditional,
    Paren,

}

#[derive(Debug, Clone)]
pub struct ParseNode {
    pub children: Vec<ParseNode>,
    pub entry: GrammarItem,
}

impl ParseNode {
    pub fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: GrammarItem::Paren,
        }
    }
}

#[cfg(test)]
mod test{
    use crate::equations::Op;

    use super::{Expression, Formula, lex};


    #[test]
    fn atom() {

        let id = Formula::new(
                    Expression::IdentityNode(true)
                );

        println!("{}", id.collapse());
        assert_eq!(true, id.collapse());
        
        let and = Formula::new(
            Expression::ConjunktionNode(
                Expression::IdentityNode(true),
                Expression::IdentityNode(true)
            )
        );

        println!("{}", and.collapse());
        assert_eq!(true, and.collapse());

        let or = Formula::new(
            Expression::DisjunktionNode(
                Expression::IdentityNode(true),
                Expression::IdentityNode(false)
            )
        );

        println!("{}", or.collapse());
        assert_eq!(true, or.collapse());

        let non = Formula::new(
            Expression::NegateNode(
                Expression::IdentityNode(false)
            )
        );
        
        println!("{}", non.collapse());
        assert_eq!(true, non.collapse());

        let implication = Formula::new(
            Expression::ImplikationNode(
                Expression::IdentityNode(false),
                Expression::IdentityNode(false)
            )
        );

        println!("{}", implication.collapse());
        assert_eq!(true, implication.collapse());

        let bi = Formula::new(
            Expression::BiconditionalNode(
                Expression::IdentityNode(false),
                Expression::IdentityNode(false)
            )
        );

        println!("{}", bi.collapse());
        assert_eq!(true, bi.collapse());

        //----------------------------------------

        let comp = Formula::new(
            Expression::ConjunktionNode(
                Expression::ConjunktionNode(
                    Expression::IdentityNode(true),
                    Expression::IdentityNode(true)),
                Expression::ConjunktionNode(
                    Expression::IdentityNode(false), 
                    Expression::NegateNode(Expression::IdentityNode(true))
                )
            ) 
        );

        println!("{}", comp.collapse());
        assert_eq!(false, comp.collapse());

        //----------------------------------------

        let broken = Formula::new(
            Expression { left_tree: Some(Box::new(Expression 
                                { left_tree: Some(Box::new(Expression 
                                    { left_tree: Some(Box::new(Expression::IdentityNode(true))), 
                                    right_tree: None, junktor: Op::Conjunktion})), 
                                right_tree: None, junktor: Op::Conjunktion})), 
                            right_tree: None, junktor: Op::Conjunktion }
                        );

        println!("{}", broken.collapse());
        assert_eq!(true, broken.collapse());

    }

    #[test]
    fn lexing() {

        let Str = String::from("(x & y) > z");

        println!("{:?}", lex(&Str).unwrap());

    }

}