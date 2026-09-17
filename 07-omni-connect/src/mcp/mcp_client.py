"""
MCP Client.

Adapted from the bootcamp src/mcp/mcp_client.py.txt to omni-connect: the one
MCP client call site fetches a customer's retail profile via the local
stdio server.
"""
import asyncio
import sys
import json
from typing import Any, Dict

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

from src.utils.config_loader import load_yaml_config
from src.utils.logger import setup_logger

logger = setup_logger(__name__)

_config = load_yaml_config("config/mcp_config.yaml")
_server_module = _config.get("server", {}).get("module", "src.mcp.mcp_server")

# Prefilled: the parameters used to launch the MCP server as a local
# subprocess over stdio. No network/paid API involved - this is local
# inter-process communication, not an external call.
_server_params = StdioServerParameters(command=sys.executable, args=["-m", _server_module])


async def _call_get_customer_profile(customer_id: str) -> Dict[str, Any]:
    """
    Open an MCP client session against the local server (launched via
    _server_params) and call its "get_customer_profile_tool" tool.
    """
    async with stdio_client(_server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            result = await session.call_tool("get_customer_profile_tool", {"customer_id": customer_id})
            return json.loads(result.content[0].text)


def fetch_customer_profile_via_mcp(customer_id: str) -> Dict[str, Any]:
    """Synchronous wrapper around _call_get_customer_profile() - the function
    workbook cells and the Streamlit app call.

    Jupyter/ipykernel already runs its own asyncio event loop, so a plain
    asyncio.run() here would raise "cannot be called from a running event
    loop" whenever this is called from a notebook cell. Running the
    coroutine on a dedicated background thread (with its own fresh event
    loop) works the same way whether called from a script, a test, or a
    notebook cell.
    """
    import concurrent.futures

    with concurrent.futures.ThreadPoolExecutor(max_workers=1) as pool:
        future = pool.submit(asyncio.run, _call_get_customer_profile(customer_id))
        return future.result()