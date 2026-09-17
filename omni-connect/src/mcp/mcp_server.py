"""
MCP Server - exposes ONE enterprise capability (customer profile lookup) as a
Model Context Protocol tool (Scope Decision 2: one clean example of the
protocol, not a full tool-surface migration).

Adapted from the bootcamp src/mcp/mcp_server.py.txt to the omni-connect
domain: the single tool calls src/services/customer_service.get_customer()
for the selected customer's retail profile.
"""
from typing import Any, Dict

from mcp.server.fastmcp import FastMCP

from src.services.customer_service import get_customer
from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

_config = load_yaml_config("config/mcp_config.yaml")
_server_name = _config.get("server", {}).get("name", "omni-connect-retail-copilot-mcp")

# Prefilled: the FastMCP server instance. Registering a tool with
# @mcp_app.tool() below only records its name/signature/docstring - it does
# NOT call the function, so this module can be imported safely even before
# the tool body is implemented.
mcp_app = FastMCP(_server_name)


@mcp_app.tool()
def get_customer_profile_tool(customer_id: str) -> Dict[str, Any]:
    """
    Look up a customer's retail profile (plan, device, billing, eligibility,
    tenure) by customer_id.
    """
    return get_customer(customer_id)


if __name__ == "__main__":
    # Runs this server over the transport declared in
    # config/mcp_config.yaml's server.transport (stdio - src/mcp/mcp_client.py
    # launches this module as a subprocess and speaks MCP to it over
    # stdin/stdout). No network call is involved (Scope Decision 2 keeps
    # this to a local, single-tool example).
    mcp_app.run(transport=_config.get("server", {}).get("transport", "stdio"))