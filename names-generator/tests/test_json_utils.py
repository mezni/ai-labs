import pytest

from utils.json_utils import extract_json


def test_extract_json_plain():
    assert extract_json('{"candidates": [{"name": "A"}]}') == {
        "candidates": [{"name": "A"}]
    }


def test_extract_json_with_fences():
    content = '```json\n{"candidates": [{"name": "A"}]}\n```'
    assert extract_json(content) == {"candidates": [{"name": "A"}]}


def test_extract_json_with_prefix_text():
    content = 'Here are some ideas:\n\n{"candidates": []}'
    assert extract_json(content) == {"candidates": []}


def test_extract_json_no_object():
    with pytest.raises(ValueError, match="No JSON object"):
        extract_json("no json here")