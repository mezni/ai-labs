from pydantic import BaseModel, Field, field_validator


class ProjectBlueprint(BaseModel):
    project_name: str = Field(
        description="A concise and meaningful name for the AI project."
    )

    business_outcome: str = Field(
        description=(
            "The primary business result the AI project should achieve. "
            "Focus on measurable business value rather than technical implementation."
        )
    )

    @field_validator("project_name", "business_outcome")
    @classmethod
    def validate_not_empty(cls, value: str) -> str:
        value = value.strip()

        if not value:
            raise ValueError("Value cannot be empty.")

        return value


class GenerateBlueprintRequest(BaseModel):
    project_idea: str = Field(
        description="The high-level idea describing the AI project."
    )

    @field_validator("project_idea")
    @classmethod
    def validate_project_idea(cls, value: str) -> str:
        value = value.strip()

        if not value:
            raise ValueError("Project idea cannot be empty.")

        return value


class GenerationTelemetryResponse(BaseModel):
    request_id: str
    model: str
    prompt_version: str
    latency_seconds: float
    input_tokens: int | None = None
    output_tokens: int | None = None
    total_tokens: int | None = None


class GenerateBlueprintResponse(BaseModel):
    blueprint: ProjectBlueprint
    telemetry: GenerationTelemetryResponse