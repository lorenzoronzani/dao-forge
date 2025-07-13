import { Card, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Plus, X } from "lucide-react";
import { Role, User } from "@/models/entities/User";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

type MembersInfoCardProps = {
    id: string;
    title: string;
    members: User[];
    onValueChange: (id: string, value: User[]) => void;
}

export const MembersInfoCard = ({ id, title, members, onValueChange }: MembersInfoCardProps) => {
    const addMember = () => {
        onValueChange(id, [...members, new User('', Role.Member)]);
    };

    const updateMember = (index: number, newId: string, newRole: Role) => {
        const updatedMembers = [...members];
        updatedMembers[index] = new User(newId, newRole);
        onValueChange(id, updatedMembers);
    };

    const removeMember = (index: number) => {
        const updatedMembers = [...members];
        updatedMembers.splice(index, 1);
        onValueChange(id, updatedMembers);
    };

    return (
        <Card>
            <CardContent className="pt-6">
                <div className="flex justify-between items-center mb-4">
                    <h3 className="text-lg font-medium">{title}</h3>
                    <Button
                        type="button"
                        variant="outline"
                        size="sm"
                        onClick={addMember}
                        className="flex items-center gap-1"
                    >
                        <Plus className="h-4 w-4" /> Add
                    </Button>
                </div>

                <div className="space-y-2">
                    {members.map((member, index) => (
                        <div key={`member-${index}`} className="flex items-center gap-2">
                            <Input
                                value={member.id}
                                onChange={(e) => updateMember(index, e.target.value, member.role)}
                                placeholder="Principal ID"
                                className="flex-1"
                            />
                            <Select
                                value={member.role}
                                onValueChange={(role: Role) => updateMember(index, member.id, role)}
                            >
                                <SelectTrigger className="w-[120px]">
                                    <SelectValue placeholder="Role" />
                                </SelectTrigger>
                                <SelectContent>
                                    {Object.values(Role).map(role => (
                                        <SelectItem key={role} value={role}>{role}</SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                            <Button
                                type="button"
                                variant="ghost"
                                size="icon"
                                onClick={() => removeMember(index)}
                            >
                                <X className="h-4 w-4" />
                            </Button>
                        </div>
                    ))}
                </div>
            </CardContent>
        </Card>
    );
}