from dataclasses import dataclass

import uuid


@dataclass(frozen=True)
class RequestContext:
    request_id: str
    trace_id: str

    @classmethod
    def create(cls) -> "RequestContext":
        return cls(
            request_id=str(uuid.uuid4()),
            trace_id=str(uuid.uuid4()),
        )

    def create_span_id(self) -> str:
        return str(uuid.uuid4())