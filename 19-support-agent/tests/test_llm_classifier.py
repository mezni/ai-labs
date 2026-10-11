from unittest.mock import patch

import pytest
from openai import APITimeoutError

from support_agent.llm_classifier import classify_ticket


@patch("support_agent.llm_classifier.create_llm_client")
@patch.dict(
    "os.environ",
    {
        "OPENAI_API_KEY": "test-key",
        "OPENAI_MODEL": "test-model",
    },
)
def test_timeout_is_propagated(mock_create_client):
    client = mock_create_client.return_value

    client.chat.completions.parse.side_effect = APITimeoutError(request=None)

    with pytest.raises(APITimeoutError):
        classify_ticket("Where is order #4821?")
