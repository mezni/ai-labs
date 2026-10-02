from mcp.server.mcpserver import MCPServer

mcp = MCPServer("mcp-lab")


@mcp.tool()
def add(a: int, b: int) -> int:
    """Add two numbers together."""
    return a + b


if __name__ == "__main__":
    mcp.run()