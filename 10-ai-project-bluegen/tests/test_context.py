from context import RequestContext


def test_request_context_generates_ids() -> None:
    context = RequestContext.create()

    assert context.request_id
    assert context.trace_id


def test_span_ids_are_unique() -> None:
    context = RequestContext.create()

    span_one = context.create_span_id()
    span_two = context.create_span_id()

    assert span_one
    assert span_two
    assert span_one != span_two


def test_request_context_is_immutable() -> None:
    context = RequestContext.create()

    try:
        context.request_id = "new-id"
        assert False
    except AttributeError:
        pass