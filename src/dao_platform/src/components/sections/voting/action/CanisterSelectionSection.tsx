import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { Principal } from "@dfinity/principal";

export type CanisterDetails = {
    canisterId: Principal;
    methodsNumber: number;
}

interface CanisterSelectionSectionProps {
    canisters: Map<string, CanisterDetails>;
    handleCanisterSelect: (value: string) => void;
}

export const CanisterSelectionSection = ({ canisters, handleCanisterSelect }: CanisterSelectionSectionProps) => {
    return (
        <VerticalLabeledComponent label="Select Canister" htmlFor="canister-select">
            <Select onValueChange={(value) => handleCanisterSelect(value)}>
                <SelectTrigger>
                    <SelectValue placeholder="Choose a canister..." />
                </SelectTrigger>
                <SelectContent>
                    {Array.from(canisters.entries()).map(([name, details]) => (
                        <SelectItem key={name} value={details.canisterId.toText()}>
                            <div className="flex items-center justify-between w-full">
                                <span>{name}</span>

                                <Badge variant="outline" className="ml-2">
                                    {details.methodsNumber} methods
                                </Badge>
                            </div>
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
        </VerticalLabeledComponent>
    );
};

export default CanisterSelectionSection;
