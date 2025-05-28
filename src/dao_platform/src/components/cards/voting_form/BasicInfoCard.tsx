import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { FileText } from "lucide-react";

type BasicInfoCardProps = {
    title: string;
    description: string;
    onValueChange: (field: string, value: string) => void;
}

export const BasicInfoCard = ({ title, description, onValueChange }: BasicInfoCardProps) => {
    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <FileText className="h-5 w-5" />
                    Basic Information
                </CardTitle>
            </CardHeader>
            <CardContent className="space-y-4">
                <VerticalLabeledComponent label="Title" htmlFor="title">
                    <Input
                        id="title"
                        value={title}
                        onChange={(e) => onValueChange(e.target.id, e.target.value)}
                        placeholder="E.g., Board Member Election"
                        required
                    />
                </VerticalLabeledComponent>

                <VerticalLabeledComponent label="Description" htmlFor="description">
                    <Textarea
                        id="description"
                        value={description}
                        onChange={(e) => onValueChange(e.target.id, e.target.value)}
                        placeholder="Provide details about the voting"
                        className="min-h-24"
                        required
                    />
                </VerticalLabeledComponent>
            </CardContent>
        </Card>
    )
}