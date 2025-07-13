import { Card, CardHeader, CardTitle, CardDescription, CardContent } from "@/components/ui/card";
import { Users } from "lucide-react";
import { Dao } from "@/models/entities/Dao";
import { Principal } from "@dfinity/principal";
import { useState } from "react";
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Role } from "@/models/entities/User";

type VotersCardProps = {
    dao: Dao;
    onValueChange: (field: string, value: Principal[]) => void;
}

export const VotersCard = ({ dao, onValueChange }: VotersCardProps) => {
    const [voterGroup, setVoterGroup] = useState<Role | undefined>();

    const handleVoterGroupChange = (value: string) => {
        setVoterGroup(value as Role);
        onValueChange('votersWhitelist', getVoters(value as Role));
    };

    const getVoters = (voterGroup: Role | undefined): Principal[] => {
        if (!voterGroup) {
            return [];
        }
        switch (voterGroup) {
            case Role.Board:
                return dao.members.filter(member => member.role === Role.Board).map(member => Principal.fromText(member.id));
            case Role.Member:
                return dao.members.map(member => Principal.fromText(member.id));
        }
    }

    return (
        <Card>
            <CardHeader>
                <CardTitle className="flex items-center gap-2">
                    <Users className="h-5 w-5" />
                    Voters
                </CardTitle>
                <CardDescription>Select who can participate in this voting</CardDescription>
            </CardHeader>
            <CardContent className="space-y-4">
                <VerticalLabeledComponent label="Voters" htmlFor="votersWhitelist">
                    <Select
                        onValueChange={handleVoterGroupChange}
                        required
                    >
                        <SelectTrigger>
                            <SelectValue placeholder="Select Voters" />
                        </SelectTrigger>
                        <SelectContent>
                            <SelectItem value={Role.Board}>At least Board ({dao.members.filter(member => member.role === Role.Board).length})</SelectItem>
                            <SelectItem value={Role.Member}>At least Member ({dao.members.length})</SelectItem>
                        </SelectContent>
                    </Select>
                </VerticalLabeledComponent>

                <div className="mt-4 border rounded-md p-3 max-h-40 overflow-y-auto bg-slate-50">
                    <h4 className="text-sm font-medium mb-2">Selected Voters ({getVoters(voterGroup).length})</h4>
                    <div className="space-y-1">
                        {getVoters(voterGroup).map((voter, index) => (
                            <div key={index} className="text-xs font-mono text-slate-700 truncate">
                                {voter.toText()}
                            </div>
                        ))}
                        {getVoters(voterGroup).length === 0 && (
                            <p className="text-xs text-slate-500">No voters selected</p>
                        )}
                    </div>
                </div>
            </CardContent>
        </Card>
    )
}

export default VotersCard;