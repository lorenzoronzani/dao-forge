import { ActionFormData } from "@/components/forms/VotingForm";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Principal } from "@dfinity/principal";
import { Settings, AlertCircle } from "lucide-react";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { DaoAssociationService } from "@/services/daoAssocationService";
import { NetworkCallService } from "@/services/networkCallService";
import MethodSelectionSection from "./MethodSelectionSection";
import MethodInformationSection from "./MethodInformationSection";
import DynamicParamsSection from "./DynamicParamsSection";
import CanisterSelectionSection, { CanisterDetails } from "./CanisterSelectionSection";
import { ICP_CANISTER_ID } from "@/constants/icp";

type ActionSectionProps = {
    daoPrincipal: Principal;
    action: ActionFormData;
    onValueChange: (field: string, value: unknown) => void;
    areCustomOptions: boolean;
};

export const ActionSection = ({ daoPrincipal, action, onValueChange, areCustomOptions }: ActionSectionProps) => {
    const { identity } = useAuthentication();

    const handleCanisterSelect = (value: string) => {
        action.canisterId = value;
        action.method = '';
        action.args = [];
        onValueChange('action', action);
    };

    const handleMethodSelect = (value: string) => {
        action.method = value;
        action.args = [];
        onValueChange('action', action);
    };

    const daoAssociationMethods = new DaoAssociationService(daoPrincipal, identity).getMethods();
    const networkCallMethods = new NetworkCallService(ICP_CANISTER_ID.NETWORK_CALL).getMethods();

    const canisters = new Map<string, CanisterDetails>();
    canisters.set('Dao association', { canisterId: daoPrincipal, methodsNumber: daoAssociationMethods.length });
    canisters.set('Network calls', { canisterId: ICP_CANISTER_ID.NETWORK_CALL, methodsNumber: networkCallMethods.length });

    const getMethods = () => {
        switch (action.canisterId) {
            case daoPrincipal.toText():
                return daoAssociationMethods;
            case ICP_CANISTER_ID.NETWORK_CALL.toText():
                return networkCallMethods;
            default:
                return [];
        }
    }

    const methods = getMethods();
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
                <CanisterSelectionSection canisters={canisters} handleCanisterSelect={handleCanisterSelect} />

                {methods.length > 0 && (
                    <MethodSelectionSection action={action} methods={methods} handleMethodSelect={handleMethodSelect} />
                )}

                {methodSig && (
                    <MethodInformationSection method={methodSig} />
                )}

                {methodSig && methodSig.parameters.length > 0 && (
                    <DynamicParamsSection action={action} onValueChange={onValueChange} methodSig={methodSig} areCustomOptions={areCustomOptions} />
                )}

                {methodSig && methodSig.parameters.length === 0 && (
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
