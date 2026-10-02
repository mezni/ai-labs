from telemetry import TelemetryEvent


class InMemoryTelemetryRecorder:

    def __init__(self) -> None:
        self.events: list[TelemetryEvent] = []

    def record(
        self,
        event: TelemetryEvent,
    ) -> None:
        self.events.append(event)