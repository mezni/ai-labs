from mcp.server.fastmcp import FastMCP

mcp = FastMCP("local-server")


@mcp.tool()
def get_customer_name(customer_id: str) -> str:
    """Get customer name by providing customer id."""
    if customer_id == "cust-001":
        return "dali"
    return "Unknown Customer"


if __name__ == "__main__":
    mcp.run(transport="stdio")
