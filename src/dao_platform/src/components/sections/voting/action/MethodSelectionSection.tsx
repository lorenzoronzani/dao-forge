import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { MethodSignature } from "@/services/canisterAnalyzerService";
import { ActionFormData } from "@/components/forms/VotingForm";
import { Badge } from "@/components/ui/badge";

interface MethodSelectionSectionProps {
    action: ActionFormData;
    methods: MethodSignature[];
    handleMethodSelect: (value: string) => void;
}

export const MethodSelectionSection = ({ action, methods, handleMethodSelect }: MethodSelectionSectionProps) => {
    return (
        <VerticalLabeledComponent label="Select Method" htmlFor="method-select">
            <Select value={action.method} onValueChange={(value) => handleMethodSelect(value)}>
                <SelectTrigger>
                    <SelectValue placeholder="Choose a method to execute..." />
                </SelectTrigger>
                <SelectContent>
                    {methods.map((method) => (
                        <SelectItem key={method.name} value={method.name}>
                            <div className="flex items-center justify-between w-full">
                                <span>{method.name}</span>

                                <Badge variant="outline" className="ml-2">
                                    {method.parametersType.length} params
                                </Badge>
                            </div>
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </VerticalLabeledComponent>
    );
};

export default MethodSelectionSection;
