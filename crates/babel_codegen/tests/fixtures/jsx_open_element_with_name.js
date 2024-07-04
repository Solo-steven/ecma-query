function testASTNode_2(node) { return node && node.name === 'className'  && node.type === 'JSXIdentifier' ;  }
function testASTNode_0(node) { return node && Array.isArray(node.attributes) && node.attributes.some(testASTNode_1) && node.type === 'JSXOpenElement' ;  }
function testASTNode_1(node) { return node && node.type === 'JSXAttribute'  && testASTNode_2(node.name);  }
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