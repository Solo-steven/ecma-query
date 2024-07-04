# ECMA Query

A DSL for JavaScript related language to creating a AST transformer for transpiler plugin, linter plugin ..etc

### Example
Using S-expression to query a JSX element with have `className` attribute
```
(query 
    (node 
        (type 'JSXOpenElement')
        (attributes array (node 
            (type 'JSXAttribute')
            (name (node 
                (type 'JSXIdentifier')
                (name 'className')
            ))
        ))
    )
)
```
It could generate Babel plugin with 

```js
function testASTNode_2(node) { return node && node.name === 'className'  && node.type === 'JSXIdentifier' ;  }
function testASTNode_0(node) { return node && Array.isArray(node.attributes) && node.attributes.some(testASTNode_1) && node.type === 'JSXOpenElement' ;  }
function testASTNode_1(node) { return node && testASTNode_2(node.name) && node.type === 'JSXAttribute' ;  }
export default function() {
	return {
		visitor: {
			'JSXOpenElement'(path) {
				const node = path.node;
				if(testASTNode_0(node)) {}
			}
		}
	}
}
```