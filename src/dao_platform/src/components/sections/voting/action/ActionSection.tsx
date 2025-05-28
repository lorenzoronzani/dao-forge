import { ActionFormData } from "@/components/forms/VotingForm";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Principal } from "@dfinity/principal";
import { Settings, AlertCircle } from "lucide-react";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { DaoAssociationService } from "@/services/daoAssocationService";
import MethodSelectionSection from "./MethodSelectionSection";
import MethodInformationSection from "./MethodInformationSection";
import DynamicParamsSection from "./DynamicParamsSection";

type ActionSectionProps = {
    action: ActionFormData;
    onValueChange: (field: string, value: unknown) => void;
    areCustomOptions: boolean;
};

export const ActionSection = ({ action, onValueChange, areCustomOptions }: ActionSectionProps) => {
    const { identity } = useAuthentication();

    const handleMethodSelect = (value: string) => {
        action.method = value;
        action.args = [];
        onValueChange('action', action);
    };

    const methods = new DaoAssociationService(Principal.from(action.canisterId), identity).getMethods();
    const methodSig = methods.find(m => m.name === action.method);

    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <Settings className="h-5 w-5" />
                    Action Configuration
                </CardTitle>
                <CardDescription>
                    Configure the action that will be executed if the voting passes
                </CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
                <MethodSelectionSection action={action} methods={methods} handleMethodSelect={handleMethodSelect} />

                {methodSig && (
                    <MethodInformationSection method={methodSig} />
                )}

                {methodSig && methodSig.parametersType.length > 0 && (
                    <DynamicParamsSection action={action} onValueChange={onValueChange} methodSig={methodSig} areCustomOptions={areCustomOptions} />
                )}

                {methodSig && methodSig.parametersType.length === 0 && (
                    <div className="p-3 bg-blue-50 rounded-lg">
                        <div className="flex items-center gap-2">
                            <AlertCircle className="h-4 w-4 text-blue-600" />
                            <span className="text-sm text-blue-800">
                                This method requires no parameters and will be executed as-is.
                            </span>
                        </div>
                    </div>
                )}
            </CardContent>
        </Card>
    );
};

export default ActionSection;
